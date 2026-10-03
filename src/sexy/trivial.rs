//! Virtual functions whose whole body is `ret` / `ret N`. The linker's identical-code folding
//! points many unrelated empty virtuals at one of these, so the DB name (the first class that
//! claimed the address) rarely matches the slot that calls it. Vtable slots with a unit
//! return that target one of these call it through a generated wrapper (tools/gen_vtables_rs.py).

/// port: 00457ad0 Sexy::Image::vfunction6
pub fn vfunction6__00457ad0() {}
/// port: 00457b60 Sexy::Image::vfunction11
pub fn vfunction11__00457b60() {}
/// port: 00457b70 Sexy::Image::vfunction12
pub fn vfunction12__00457b70() {}
/// port: 00457b80 Sexy::Image::vfunction9
pub fn vfunction9__00457b80() {}
/// port: 00457b90 Sexy::Font::vfunction11
pub fn vfunction11__00457b90() {}
/// port: 0046c690 Sexy::ResourceManager::vfunction14
pub fn vfunction14__0046c690() {}
/// port: 0046e400 Sexy::MusicInterface::vfunction13
pub fn vfunction13__0046e400() {}
/// port: 0046eb50 Sexy::DialogListener::vfunction1
pub fn vfunction1__0046eb50() {}
/// port: 00486bf0 Sexy::ResourceManager::BaseRes::vfunction2
pub fn vfunction2__00486bf0() {}
/// port: 0049b2f0 Sexy::MusicInterface::vfunction12
pub fn vfunction12__0049b2f0() {}
/// port: 0049b300 Sexy::Image::vfunction3
pub fn vfunction3__0049b300() {}
/// port: 00519120 Sexy::CheckboxListener::vfunction1
pub fn vfunction1__00519120() {}
/// port: 0051a440 Sexy::ListListener::vfunction2
pub fn vfunction2__0051a440() {}
/// port: 00531e70 Sexy::SliderListener::vfunction1
pub fn vfunction1__00531e70() {}
/// port: 004d5350 Sexy::EditListener::vfunction2
/// `mov al, 1; ret 8`: EditListener's `AllowKey` / `AllowText` defaults (`return true`).
pub fn vfunction2__004d5350() -> bool {
    true
}
/// port: 004d8c10 Sexy::Grubber::vfunction59
/// `push ebp; mov ebp, esp; pop ebp; jmp 0046eb50`: a tail jump to the empty `ret 8`.
pub fn vfunction59__004d8c10() {}
/// port: 00401b20 std::codecvt<char,char,int>::vfunction2
/// `return true` (folded; EditWidget's `WantsFocus` lands here).
pub fn vfunction2__00401b20() -> bool {
    true
}
