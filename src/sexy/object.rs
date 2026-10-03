//! The object model: every heap object of the original (widgets, game objects, images,
//! the app) lives in one arena inside [`G`], addressed by [`Ptr`]. Classes are composed
//! from the same `<Part>_data` blocks the database lists for them, with field names taken
//! verbatim from the decompiled source (`offset_0xNN` where the database types a member,
//! `field_0xNN` for bytes it left untyped; see tools/off.py).

use crate::sexy::prelude::*;
use crate::sexy::std_list::{ListIter, StdList};

/// `WidgetContainer_data` (object offset 0x4, 80 bytes).
#[derive(Clone, Debug, Default)]
pub struct WidgetContainer_data {
    /// `mWidgets` (std::list<Widget*>, 0x4..0xc: head node and size).
    pub offset_0x4: StdList<Ptr>,
    /// `mWidgetManager`.
    pub offset_0xc: Ptr,
    /// `mParent`.
    pub offset_0x10: Ptr,
    /// `mUpdateIteratorModified`.
    pub offset_0x14: bool,
    /// `mUpdateIterator` (0x18 is the checked iterator's container pointer, 0x1c its node).
    pub offset_0x1c: ListIter,
    /// `mLastWMUpdateCount`.
    pub offset_0x20: i32,
    /// `mUpdateCnt`.
    pub offset_0x24: i32,
    /// `mDirty`.
    pub offset_0x28: bool,
    /// `mX`.
    pub offset_0x2c: i32,
    /// `mY`.
    pub offset_0x30: i32,
    /// `mWidth`.
    pub offset_0x34: i32,
    /// `mHeight`.
    pub offset_0x38: i32,
    /// `mHasAlpha`.
    pub offset_0x3c: bool,
    /// `mClip`.
    pub offset_0x3d: bool,
    /// `mWidgetFlagsMod.mAddFlags`.
    pub offset_0x40: u32,
    /// `mWidgetFlagsMod.mRemoveFlags`.
    pub offset_0x44: u32,
    /// `mPriority`.
    pub offset_0x48: i32,
    /// `mZOrder`.
    pub offset_0x4c: i32,
}

/// `Widget_data` (object offset 0x54, 52 bytes).
#[derive(Clone, Debug, Default)]
pub struct Widget_data {
    /// `mVisible`.
    pub offset_0x0: bool,
    /// `mMouseVisible`.
    pub offset_0x1: bool,
    /// `mDisabled`.
    pub offset_0x2: bool,
    /// `mHasFocus`.
    pub offset_0x3: bool,
    /// `mIsDown`.
    pub offset_0x4: bool,
    /// `mIsOver`.
    pub offset_0x5: bool,
    /// `mHasTransparencies`.
    pub offset_0x6: bool,
    /// `mColors` (std::vector<Color>, 0x8..0x14).
    pub offset_0xc: Vec<Color>,
    /// `mMouseInsets` (left, top, right, bottom).
    pub field_0x18: [i32; 4],
    /// `mDoFinger`.
    pub offset_0x28: bool,
    /// `mWantsFocus`.
    pub offset_0x29: bool,
    /// `mTabPrev`.
    pub offset_0x2c: Ptr,
    /// `mTabNext`.
    pub offset_0x30: Ptr,
}

/// A widget-derived object: the two framework parts plus the class-specific rest.
#[derive(Debug)]
pub struct WidgetObj {
    pub wc: WidgetContainer_data,
    pub w: Widget_data,
    pub ext: WExt,
}

/// Class-specific data of widget-derived objects.
#[derive(Debug)]
pub enum WExt {
    /// `Sexy::Widget` itself or a class that adds no data.
    None,
    WidgetManager(Box<crate::sexy::widget_manager::WidgetManager_data>),
    Button(Box<crate::sexy::button_widget::ButtonExt>),
    GameObject(Box<GameObjectExt>),
    Board(Box<crate::game::board::Board_data>),
    TitleScreen(Box<crate::game::title_screen::TitleScreen_data>),
    GameSelector(Box<crate::game::game_selector::GameSelector_data>),
    /// `GameSelectorOverlay`: a plain Widget plus its owner at +0x88.
    GameSelectorOverlay(Ptr),
    HelpScreen(Box<crate::game::help_screen::HelpScreen_data>),
    HatchScreen(Box<crate::game::hatch_screen::HatchScreen_data>),
    Dialog(Box<crate::sexy::dialog::DialogExt>),
    EditWidget(Box<crate::sexy::edit_widget::EditWidget_data>),
    MyLabel(Box<crate::game::board_parts::MyLabelWidget_data>),
    Message(Box<crate::game::board_parts::MessageWidget_data>),
    BoardOverlay(Box<crate::game::board_parts::BoardOverlay_data>),
    Slider(Box<crate::sexy::slider::Slider_data>),
    Scrollbar(Box<crate::sexy::scrollbar::ScrollbarWidget_data>),
    List(Box<crate::sexy::list_widget::ListWidget_data>),
    Checkbox(Box<crate::sexy::checkbox::Checkbox_data>),
    Store(Box<crate::game::store::StoreScreen_data>),
    SimSetup(Box<crate::game::sim_setup::SimSetupScreen_data>),
    SimFish(Box<crate::game::sim_fish::SimFishScreen_data>),
    HighScoreScreen(Box<crate::game::high_score_screen::HighScoreScreen_data>),
    PetsScreen(Box<crate::game::pets_screen::PetsScreen_data>),
    InterludeScreen(Box<crate::game::interlude_screen::InterludeScreen_data>),
    BonusScreen(Box<crate::game::bonus_screen::BonusScreen_data>),
    TankScreen(Box<crate::game::tank_screen::TankScreen_data>),
    StoryScreen(Box<crate::game::story_screen::StoryScreen_data>),
}

/// `GameObject_data` plus the derived class's own parts.
#[derive(Debug)]
pub struct GameObjectExt {
    pub go: crate::game::game_object::GameObject_data,
    pub sub: GoSub,
}

/// Parts below `Sexy::GameObject`.
#[derive(Debug)]
pub enum GoSub {
    None,
    Food(crate::game::food::Food_data),
    Fish(crate::game::fish::Fish_data),
    Shadow(crate::game::shadow::Shadow_data),
    Coin(crate::game::coin::Coin_data),
    Shot(crate::game::shot::Shot_data),
    DeadFish(crate::game::dead_fish::DeadFish_data),
    Alien(crate::game::alien::Alien_data),
    Warp(crate::game::warp::Warp_data),
    DeadAlien(crate::game::dead_alien::DeadAlien_data),
    OtherTypePet(crate::game::other_pet::OtherTypePet_data),
    /// A `FishTypePet` is a `Fish` with its own data after the fish's.
    FishTypePet(crate::game::fish::Fish_data, crate::game::fish_type_pet::FishTypePet_data),
    BoxingGlove(crate::game::fish_type_pet::BoxingGlove_data),
    Penta(crate::game::penta::Penta_data),
    Grubber(crate::game::grubber::Grubber_data),
    Larva(crate::game::larva::Larva_data),
    Breeder(crate::game::breeder::Breeder_data),
    Missle(crate::game::missle::Missle_data),
    Bilaterus(crate::game::bilaterus::Bilaterus_data),
    /// A `BiFish` is a `Fish` with two ints after the fish's data.
    BiFish(crate::game::fish::Fish_data, crate::game::vt_fish::BiFish_data),
    BilaterusHead(crate::game::bilaterus::BilaterusHead_data),
    BilaterusBone(crate::game::bilaterus::BilaterusBone_data),
}

/// Non-widget objects.
#[derive(Debug)]
pub enum Node {
    Widget(WidgetObj),
    Font(Box<crate::sexy::image_font::ImageFont>),
    App(Box<crate::game::app::WinFishApp>),
    Image(Box<crate::sexy::image::Image>),
    MTRand(Box<MTRand>),
    Profile(Box<crate::game::profile::PlayerProfile>),
    ProfileMgr(Box<crate::game::profile_mgr::ProfileMgr>),
    HighScoreMgr(Box<crate::game::high_score::HighScoreMgr_data>),
    StarField(Box<crate::game::board_parts::StarField_data>),
    BubbleMgr(Box<crate::game::board_parts::BubbleMgr_data>),
    TimedMessages(Box<crate::game::timed_messages::TimedMessages>),
    /// `mMusicInterface` (its state lives in `G::music`).
    MusicInterface,
}

/// One live object: its current vftable (as in C++, it changes during construction and
/// destruction) and its data.
pub struct Obj {
    pub vt: Option<&'static VTable>,
    pub node: Node,
}

impl std::fmt::Debug for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Obj({}, {:?})", self.vt.map_or("-", |v| v.class), self.node)
    }
}

impl G {
    /// `operator new` + placement: stores `obj` and returns its pointer.
    pub fn alloc(&mut self, obj: Obj) -> Ptr {
        // An image loaded from a file: its HD art is requested (port addition, `crate::sexy::hd`).
        let file_image = matches!(&obj.node, Node::Image(i) if !i.field_0x4.is_empty());
        self.objs.push(Some(obj));
        let p = (self.objs.len() - 1) as Ptr;
        if file_image {
            self.hd.created.push(p);
        }
        p
    }

    /// `operator delete` (`_free`): drops the object. Later use of `p` panics.
    pub fn free(&mut self, p: Ptr) {
        let slot = self.objs.get_mut(p as usize).unwrap_or_else(|| panic!("free of bad pointer {p}"));
        assert!(slot.is_some(), "double free of {p}");
        *slot = None;
    }

    pub fn is_live(&self, p: Ptr) -> bool {
        p != 0 && self.objs.get(p as usize).is_some_and(|o| o.is_some())
    }

    pub fn obj(&mut self, p: Ptr) -> &mut Obj {
        match self.objs.get_mut(p as usize) {
            Some(Some(o)) => o,
            _ => panic!("dereference of dead or null pointer {p}"),
        }
    }

    pub fn obj_ref(&self, p: Ptr) -> &Obj {
        match self.objs.get(p as usize) {
            Some(Some(o)) => o,
            _ => panic!("dereference of dead or null pointer {p}"),
        }
    }

    /// The object's vftable (`*(void**)p`).
    pub fn vt(&self, p: Ptr) -> &'static VTable {
        self.obj_ref(p).vt.unwrap_or_else(|| panic!("object {p} has no vftable"))
    }

    pub fn set_vt(&mut self, p: Ptr, vt: &'static VTable) {
        self.obj(p).vt = Some(vt);
    }

    pub fn widget(&mut self, p: Ptr) -> &mut WidgetObj {
        match &mut self.obj(p).node {
            Node::Widget(w) => w,
            n => panic!("{p} is not a widget: {n:?}"),
        }
    }

    pub fn widget_ref(&self, p: Ptr) -> &WidgetObj {
        match &self.obj_ref(p).node {
            Node::Widget(w) => w,
            n => panic!("{p} is not a widget: {n:?}"),
        }
    }

    /// `WidgetContainer_data` of any widget.
    pub fn wc(&mut self, p: Ptr) -> &mut WidgetContainer_data {
        &mut self.widget(p).wc
    }

    /// `Widget_data` of any widget.
    pub fn w(&mut self, p: Ptr) -> &mut Widget_data {
        &mut self.widget(p).w
    }

    pub fn go_ext(&mut self, p: Ptr) -> &mut GameObjectExt {
        match &mut self.widget(p).ext {
            WExt::GameObject(e) => e,
            e => panic!("{p} is not a GameObject: {e:?}"),
        }
    }

    /// `GameObject_data` of any game object.
    pub fn go(&mut self, p: Ptr) -> &mut crate::game::game_object::GameObject_data {
        &mut self.go_ext(p).go
    }

    pub fn food(&mut self, p: Ptr) -> &mut crate::game::food::Food_data {
        match &mut self.go_ext(p).sub {
            GoSub::Food(d) => d,
            s => panic!("{p} is not a Food: {s:?}"),
        }
    }

    pub fn image(&mut self, p: Ptr) -> &mut crate::sexy::image::Image {
        match &mut self.obj(p).node {
            Node::Image(i) => i,
            n => panic!("{p} is not an Image: {n:?}"),
        }
    }

    pub fn mtrand(&mut self, p: Ptr) -> &mut MTRand {
        match &mut self.obj(p).node {
            Node::MTRand(r) => r,
            n => panic!("{p} is not an MTRand: {n:?}"),
        }
    }
}

/// Virtual call helpers: `vcall!(g, p, w.vfunction23)` calls slot 23 of `p`'s vftable with
/// `(g, p, args...)`, i.e. `(**(code **)(*p + 0x58))(p, args...)`.
#[macro_export]
macro_rules! vcall {
    ($g:expr, $p:expr, w.$slot:ident $(, $arg:expr)* $(,)?) => {{
        let __p: $crate::sexy::types::Ptr = $p;
        let __f = $g.vt(__p).w.$slot;
        __f($g, __p $(, $arg)*)
    }};
    ($g:expr, $p:expr, $fam:ident.$slot:ident $(, $arg:expr)* $(,)?) => {{
        let __p: $crate::sexy::types::Ptr = $p;
        let __f = $g.vt(__p).$fam.as_ref().expect(concat!("vftable has no `", stringify!($fam), "` part")).$slot;
        __f($g, __p $(, $arg)*)
    }};
}
