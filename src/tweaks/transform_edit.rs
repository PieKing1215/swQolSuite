use anyhow::Context;
use hudhook::imgui::Key;
use memory_rs::{
    generate_aob_pattern,
    internal::injections::{Inject, Injection},
};
use retour::GenericDetour;

use super::{InjectAt, MemoryRegionExt, Tweak};

#[repr(C)]
struct Transform {
    _unimportant: [u8; 0x9C],
    rotation_mat3i_cur: [i32; 9],
    rotation_mat3f_cur: [f32; 9],
}

type UpdateQuaternionFn = extern "fastcall" fn(*mut Transform);
type EditorDestructorFn = extern "fastcall" fn(*mut (), *mut ());

static mut UPDATE_QUATERNION_FN: Option<UpdateQuaternionFn> = None;
static mut EDITOR_DESTRUCTOR_FN: Option<EditorDestructorFn> = None;

static mut TRANSFORM: Option<*mut Transform> = None;

pub struct TransformEditTweak {
    _update_quaternion_detour: GenericDetour<UpdateQuaternionFn>,
    _editor_destructor_detour: GenericDetour<EditorDestructorFn>,
    disable_quaternion_slerp: bool,
    disable_quaternion_slerp_injection: Injection,
}

impl TransformEditTweak {
    fn check_orthonormal(&mut self, matrix: &[i32; 9]) {
        let orthonormal = is_orthonormal(matrix);
        if orthonormal == self.disable_quaternion_slerp {
            self.disable_quaternion_slerp = !self.disable_quaternion_slerp;
            if self.disable_quaternion_slerp {
                self.disable_quaternion_slerp_injection.inject();
            } else {
                self.disable_quaternion_slerp_injection.remove_injection();
                Self::force_update_quaternion();
            }
        }
    }

    fn reset_transform(&mut self) {
        if let Some(tr) = unsafe { TRANSFORM } {
            #[allow(clippy::cast_precision_loss)]
            unsafe {
                (*tr).rotation_mat3i_cur = [1, 0, 0, 0, 1, 0, 0, 0, 1];
                (*tr).rotation_mat3f_cur = (*tr).rotation_mat3i_cur.map(|i| i as _);
                self.check_orthonormal(&(*tr).rotation_mat3i_cur);
                Self::force_update_quaternion();
            }
        }
    }

    fn force_update_quaternion() {
        unsafe {
            if let (Some(update_quaternion), Some(tr)) = (UPDATE_QUATERNION_FN, TRANSFORM) {
                update_quaternion(tr);
            }
        }
    }
}

impl Tweak for TransformEditTweak {
    fn new(builder: &mut super::TweakBuilder) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let update_quaternion_detour = unsafe {
            extern "fastcall" fn hook(tr: *mut Transform) {
                unsafe {
                    let update_quaternion: UpdateQuaternionFn =
                        UPDATE_QUATERNION_FN.unwrap_unchecked();
                    update_quaternion(tr);

                    #[allow(clippy::cast_precision_loss)]
                    if !is_orthonormal(&(*tr).rotation_mat3i_cur) {
                        (*tr).rotation_mat3f_cur = (*tr).rotation_mat3i_cur.map(|i| i as _);
                    }
                    TRANSFORM = Some(tr);
                }
            }

            // update quaternion function
            #[rustfmt::skip]
            let memory_pattern = generate_aob_pattern![
                0x40, 0x53,                                                // PUSH       RBX
                0x48, 0x81, 0xec, 0x80, 0x00, 0x00, 0x00,                  // SUB        RSP,0x80
                0x48, 0x8b, 0x05, 0x00, 0x2f, 0x4c, 0x00,                  // MOV        RAX,qword ptr [DAT_140c40010]                    = 00002B992DDFA232h
                0x48, 0x33, 0xc4,                                          // XOR        RAX,RSP
                0x48, 0x89, 0x44, 0x24, 0x70,                              // MOV        qword ptr [RSP + local_18],RAX
                0x48, 0x8b, 0xd9,                                          // MOV        RBX,transform
                0xc7, 0x81, 0x1c, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 // MOV        dword ptr [RCX + transform->field33_0x11c],0x0

            ];
            let update_quaternion_fn_addr = builder
                .region
                .scan_aob_single(&memory_pattern)
                .context("Error finding dev mode addr")?;

            let det = GenericDetour::new(
                std::mem::transmute::<usize, UpdateQuaternionFn>(update_quaternion_fn_addr),
                hook,
            )?;
            UPDATE_QUATERNION_FN = Some(std::mem::transmute::<&(), UpdateQuaternionFn>(
                det.trampoline(),
            ));

            det
        };

        let editor_destructor_detour = unsafe {
            extern "fastcall" fn hook(editor: *mut (), param_2: *mut ()) {
                unsafe {
                    TRANSFORM = None;
                    let editor_destructor: EditorDestructorFn =
                        EDITOR_DESTRUCTOR_FN.unwrap_unchecked();
                    editor_destructor(editor, param_2);
                }
            }

            // vehicle editor destructor
            #[rustfmt::skip]
            let memory_pattern = generate_aob_pattern![
                0x48, 0x89, 0x5c, 0x24, 0x08, // MOV        qword ptr [RSP + local_res8],RBX
                0x57,                         // PUSH       RDI
                0x48, 0x83, 0xec, 0x20,       // SUB        RSP,0x20
                0x8b, 0xda,                   // MOV        EBX,param_2
                0x48, 0x8b, 0xf9,             // MOV        RDI,param_1
                0xe8, _, _, _, _,             // CALL       _
                0xf6, 0xc3, 0x01,             // TEST       BL,0x1
                0x74, _,                      // JZ         _
                0xba, 0xe8, 0x9c, 0x00, 0x00  // MOV        param_2,0x9ce8
            ];
            let editor_destructor_fn_addr = builder
                .region
                .scan_aob_single(&memory_pattern)
                .context("Error finding dev mode addr")?;

            let det = GenericDetour::new(
                std::mem::transmute::<usize, EditorDestructorFn>(editor_destructor_fn_addr),
                hook,
            )?;
            EDITOR_DESTRUCTOR_FN = Some(std::mem::transmute::<&(), EditorDestructorFn>(
                det.trampoline(),
            ));

            det
        };

        #[rustfmt::skip]
        let disable_quaternion_slerp_injection = builder.injection(
            // call that sets display matrix to interpolated quaternion
            generate_aob_pattern![
                0x48, 0x8d, 0x93, 0xc0, 0x00, 0x00, 0x00, // LEA        RDX,[RBX + 0xc0]
                0x48, 0x8d, 0x4c, 0x24, 0x20,             // LEA        RCX,[RSP + 0x20]
                0xe8, _, _, _, _                          // CALL       quaternion_to_matrix3f
            ],
            // NOP the CALL
            vec![0x90; 5],
            InjectAt::End,
        ).context("Error finding quaternion slerp copy addr")?;

        unsafe {
            update_quaternion_detour.enable()?;
            editor_destructor_detour.enable()?;
        }

        Ok(Self {
            _update_quaternion_detour: update_quaternion_detour,
            _editor_destructor_detour: editor_destructor_detour,
            disable_quaternion_slerp: false,
            disable_quaternion_slerp_injection,
        })
    }

    fn uninit(&mut self) -> anyhow::Result<()> {
        self.reset_transform();

        Ok(())
    }

    fn render(&mut self, ui: &hudhook::imgui::Ui) {
        if let Some(tr) = unsafe { TRANSFORM } {
            ui.text("Editor placement transform");
            if ui.is_item_hovered() {
                ui.tooltip_text("These numbers represent the rotation matrix of the component being placed (same as in xml).\nYou can also increment (or hold alt to decrement) using the Numpad (make sure NumLock is on).\nNumpad 0 resets the matrix.");
            }

            let mut next = unsafe { (*tr).rotation_mat3i_cur };
            let mut changed = false;
            #[allow(clippy::identity_op)]
            for r in 0..3 {
                ui.set_next_item_width(80.0);
                if ui
                    .input_int(format!("{}", r * 3 + 1), &mut next[r * 3 + 0])
                    .build()
                {
                    changed = true;
                }
                ui.same_line();
                ui.set_next_item_width(80.0);
                if ui
                    .input_int(format!("{}", r * 3 + 2), &mut next[r * 3 + 1])
                    .build()
                {
                    changed = true;
                }
                ui.same_line();
                ui.set_next_item_width(80.0);
                if ui
                    .input_int(format!("{}", r * 3 + 3), &mut next[r * 3 + 2])
                    .build()
                {
                    changed = true;
                }
            }
            if changed {
                #[allow(clippy::cast_precision_loss)]
                unsafe {
                    (*tr).rotation_mat3i_cur = next;
                    (*tr).rotation_mat3f_cur = next.map(|i| i as _);
                }
            }

            self.check_orthonormal(&next);
        }
    }

    fn constant_render(&mut self, ui: &hudhook::imgui::Ui) {
        let mut update = |idx: u8, add: i32| {
            if let Some(tr) = unsafe { TRANSFORM } {
                #[allow(clippy::cast_precision_loss)]
                unsafe {
                    (*tr).rotation_mat3i_cur[idx as usize] =
                        (*tr).rotation_mat3i_cur[idx as usize].saturating_add(add);
                    (*tr).rotation_mat3f_cur = (*tr).rotation_mat3i_cur.map(|i| i as _);
                    self.check_orthonormal(&(*tr).rotation_mat3i_cur);
                }
            }
        };

        let add = if ui.is_key_down(Key::LeftAlt) { -1 } else { 1 };

        if ui.is_key_pressed(Key::Keypad7) {
            update(0, add);
        } else if ui.is_key_pressed(Key::Keypad8) {
            update(1, add);
        } else if ui.is_key_pressed(Key::Keypad9) {
            update(2, add);
        } else if ui.is_key_pressed(Key::Keypad4) {
            update(3, add);
        } else if ui.is_key_pressed(Key::Keypad5) {
            update(4, add);
        } else if ui.is_key_pressed(Key::Keypad6) {
            update(5, add);
        } else if ui.is_key_pressed(Key::Keypad1) {
            update(6, add);
        } else if ui.is_key_pressed(Key::Keypad2) {
            update(7, add);
        } else if ui.is_key_pressed(Key::Keypad3) {
            update(8, add);
        } else if ui.is_key_pressed(Key::Keypad0) {
            self.reset_transform();
        }
    }
}

fn is_orthonormal(matrix: &[i32; 9]) -> bool {
    fn dot(a: &[i32], b: &[i32]) -> i32 {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    }

    fn magnitude_sq(v: &[i32]) -> i32 {
        dot(v, v)
    }

    fn det(matrix: &[i32; 9]) -> i32 {
        matrix[0] * (matrix[4] * matrix[8] - matrix[7] * matrix[5])
            - matrix[1] * (matrix[3] * matrix[8] - matrix[5] * matrix[6])
            + matrix[2] * (matrix[3] * matrix[7] - matrix[4] * matrix[6])
    }

    let col1 = [matrix[0], matrix[3], matrix[6]];
    let col2 = [matrix[1], matrix[4], matrix[7]];
    let col3 = [matrix[2], matrix[5], matrix[8]];

    let is_normalized =
        magnitude_sq(&col1) == 1 && magnitude_sq(&col2) == 1 && magnitude_sq(&col3) == 1;
    let is_orthogonal = dot(&col1, &col2) == 0 && dot(&col1, &col3) == 0 && dot(&col2, &col3) == 0;
    let is_det_positive = det(matrix) >= 0;

    is_normalized && is_orthogonal && is_det_positive
}
