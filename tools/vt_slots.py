"""Virtual slot signatures per vtable family (used by gen_vtables_rs.py).

Slot N lives at vtable offset (N-1)*4, exactly as in port/vtables.csv and the database's
`<Class>_vftable` comments ("... virtual function #N"). Meanings follow the PopCap
Framework headers the game was built against (MSVC lists overloads of one name together,
in reverse declaration order); they are documentation only, the slot number is what binds.

Each entry: (slot, meaning, rust parameter list after `g: &mut G, this: Ptr`, return type)
"""

WIDGET = [
    (1, 'deleting destructor', 'flags: u8', 'Ptr'),
    (2, 'GetRect', '', 'Rect'),
    (3, 'Intersects(WidgetContainer*)', 'other: Ptr', 'bool'),
    (4, 'AddWidget(Widget*)', 'widget: Ptr', '()'),
    (5, 'RemoveWidget(Widget*)', 'widget: Ptr', '()'),
    (6, 'HasWidget(Widget*)', 'widget: Ptr', 'bool'),
    (7, 'DisableWidget(Widget*, bool)', 'widget: Ptr, disabled: bool', '()'),
    (8, 'RemoveAllWidgets(bool doDelete, bool recursive)', 'do_delete: bool, recursive: bool', '()'),
    (9, 'SetFocus(Widget*)', 'widget: Ptr', '()'),
    (10, 'IsBelow(Widget*, Widget*)', 'w1: Ptr, w2: Ptr', 'bool'),
    (11, 'MarkAllDirty', '', '()'),
    (12, 'BringToFront(Widget*)', 'widget: Ptr', '()'),
    (13, 'BringToBack(Widget*)', 'widget: Ptr', '()'),
    (14, 'PutBehind(Widget*, Widget* ref)', 'widget: Ptr, reference: Ptr', '()'),
    (15, 'PutInfront(Widget*, Widget* ref)', 'widget: Ptr, reference: Ptr', '()'),
    (16, 'GetAbsPos', '', 'Point'),
    (17, 'MarkDirty(WidgetContainer*)', 'widget: Ptr', '()'),
    (18, 'MarkDirty', '', '()'),
    (19, 'MarkDirtyFull(WidgetContainer*)', 'widget: Ptr', '()'),
    (20, 'MarkDirtyFull', '', '()'),
    (21, 'AddedToManager(WidgetManager*)', 'manager: Ptr', '()'),
    (22, 'RemovedFromManager(WidgetManager*)', 'manager: Ptr', '()'),
    (23, 'Update', '', '()'),
    (24, 'UpdateAll(ModalFlags*)', 'flags: &mut ModalFlags', '()'),
    (25, 'UpdateF(float)', 'frac: f32', '()'),
    (26, 'UpdateFAll(ModalFlags*, float)', 'flags: &mut ModalFlags, frac: f32', '()'),
    (27, 'Draw(Graphics*)', 'gfx: &mut Graphics', '()'),
    (28, 'DrawAll(ModalFlags*, Graphics*)', 'flags: &mut ModalFlags, gfx: &mut Graphics', '()'),
    (29, 'SysColorChangedAll', '', '()'),
    (30, 'SysColorChanged', '', '()'),
    (31, 'OrderInManagerChanged', '', '()'),
    (32, 'SetVisible(bool)', 'visible: bool', '()'),
    (33, 'SetColors(int[][4], int)', 'colors: &[[i32; 4]]', '()'),
    (34, 'SetColors(int[][3], int)', 'colors: &[[i32; 3]]', '()'),
    (35, 'SetColor(int, const Color&)', 'idx: i32, color: Color', '()'),
    (36, 'GetColor(int, const Color& default)', 'idx: i32, default: Color', 'Color'),
    (37, 'GetColor(int)', 'idx: i32', 'Color'),
    (38, 'SetDisabled(bool)', 'disabled: bool', '()'),
    (39, 'ShowFinger(bool)', 'on: bool', '()'),
    (40, 'Resize(Rect)', 'rect: Rect', '()'),
    (41, 'Resize(int x, int y, int w, int h)', 'x: i32, y: i32, w: i32, h: i32', '()'),
    (42, 'Move(int x, int y)', 'x: i32, y: i32', '()'),
    (43, 'WantsFocus', '', 'bool'),
    (44, 'DrawOverlay(Graphics*, int priority)', 'gfx: &mut Graphics, priority: i32', '()'),
    (45, 'DrawOverlay(Graphics*)', 'gfx: &mut Graphics', '()'),
    (46, 'GotFocus', '', '()'),
    (47, 'LostFocus', '', '()'),
    (48, 'KeyChar(char)', 'c: u8', '()'),
    (49, 'KeyDown(KeyCode)', 'key: i32', '()'),
    (50, 'KeyUp(KeyCode)', 'key: i32', '()'),
    (51, 'MouseEnter', '', '()'),
    (52, 'MouseLeave', '', '()'),
    (53, 'MouseMove(int x, int y)', 'x: i32, y: i32', '()'),
    (54, 'MouseDown(int x, int y, int btn, int clicks)', 'x: i32, y: i32, btn: i32, clicks: i32', '()'),
    (55, 'MouseDown(int x, int y, int clicks)', 'x: i32, y: i32, clicks: i32', '()'),
    (56, 'MouseUp(int x, int y, int btn, int clicks)', 'x: i32, y: i32, btn: i32, clicks: i32', '()'),
    (57, 'MouseUp(int x, int y, int clicks)', 'x: i32, y: i32, clicks: i32', '()'),
    (58, 'MouseUp(int x, int y)', 'x: i32, y: i32', '()'),
    (59, 'MouseDrag(int x, int y)', 'x: i32, y: i32', '()'),
    (60, 'MouseWheel(int delta)', 'delta: i32', '()'),
    (61, 'IsPointVisible(int x, int y)', 'x: i32, y: i32', 'bool'),
    (62, 'WriteCenteredLine(Graphics*, int offset, const string&, Color, Color, const Point& shadow) -> Rect', 'gfx: &mut Graphics, offset: i32, line: &[u8], c1: Color, c2: Color, shadow: Point', 'Rect'),
    (63, 'WriteCenteredLine(Graphics*, int offset, const string&) -> Rect', 'gfx: &mut Graphics, offset: i32, line: &[u8]', 'Rect'),
    (64, 'WriteString(Graphics*, const string&, int x, int y, int width, int justify, bool draw, int offset, int length)', 'gfx: &mut Graphics, s: &[u8], x: i32, y: i32, width: i32, justify: i32, draw: bool, offset: i32, length: i32', 'i32'),
    (65, 'WriteWordWrapped(Graphics*, const Rect&, const string&, int lineSpacing, int justify)', 'gfx: &mut Graphics, rect: Rect, s: &[u8], spacing: i32, justify: i32', 'i32'),
    (66, 'GetWordWrappedHeight(Graphics*, int width, const string&, int lineSpacing)', 'gfx: &mut Graphics, width: i32, s: &[u8], spacing: i32', 'i32'),
    (67, 'GetNumDigits(int)', 'n: i32', 'i32'),
    (68, 'WriteNumberFromStrip(Graphics*, int, int x, int y, Image*, int spacing)', 'gfx: &mut Graphics, n: i32, x: i32, y: i32, image: Ptr, spacing: i32', '()'),
    (69, 'Contains(int x, int y)', 'x: i32, y: i32', 'bool'),
    (70, 'GetInsetRect', '', 'Rect'),
]

# Sexy::GameObject and everything derived from it (Fish, Coin, Food, aliens, pets ...).
GAME_OBJECT = [
    (71, 'GameObject virtual #71 (count a kill into the stats array)', 'stats: &mut [i32]', '()'),
    (72, 'GameObject virtual #72 (base returns 0)', '', 'i32'),
    (73, 'GameObject virtual #73 (sell/drop value)', '', 'i32'),
    (74, 'GameObject virtual #74', 'arg: i32', '()'),
    (75, 'GameObject virtual #75 (base empty)', '', '()'),
    (76, 'GameObject virtual #76 (remove from board)', '', '()'),
    (77, 'GameObject virtual #77', 'a: i32, b: i32', '()'),
    (78, 'GameObject virtual #78', 'arg: i32', '()'),
    (79, 'GameObject virtual #79', '', '()'),
    (80, 'DrawIcon(Graphics*, int pose)', 'gfx: &mut Graphics, pose: i32', '()'),
    (81, 'Sync(DataSync&)', 'sync: &mut DataSync', 'crate::sexy::data_sync::SyncResult'),
]

# Sexy::Fish and subclasses (slots after GameObject's 81).
FISH = [
    (82, 'Fish virtual #82 (coin timer: drop a coin)', '', '()'),
    (83, 'Fish virtual #83 (hunger; true while chasing food)', '', 'bool'),
    (84, 'Fish virtual #84 (draw the body)', 'gfx: &mut Graphics, mirror: bool', '()'),
    (85, 'Fish virtual #85 (swim toward food; true when there is food)', '', 'bool'),
    (86, 'Fish virtual #86 (nearest food)', '', 'Ptr'),
    (87, 'Fish virtual #87 (eat food in reach)', '', '()'),
    (88, 'Fish virtual #88 (remove from the board)', 'remove_shadow: bool', '()'),
    (89, 'Fish virtual #89 (die, leaving a dead fish)', 'sound: bool', '()'),
    (90, 'Fish virtual #90 (animate)', '', '()'),
    (91, 'Fish virtual #91 (color from a seed)', 'seed: i32, flag: bool', '()'),
]

# Sexy::ButtonWidget and subclasses (HyperlinkWidget, DialogButton, MenuButtonWidget, ...).
BUTTON = [
    (71, 'DrawButtonImage(Graphics*, Image*, const Rect&, int x, int y)', 'gfx: &mut Graphics, image: Ptr, rect: Rect, x: i32, y: i32', '()'),
    (72, 'SetFont(Font*)', 'font: Ptr', '()'),
    (73, 'IsButtonDown()', '', 'bool'),
]

# The ButtonListener interface (a class's `vftable_for_ButtonListener`, 7 slots).
BUTTON_LISTENER = [
    (1, 'ButtonPress(int id, int clickCount)', 'id: i32, clicks: i32', '()'),
    (2, 'ButtonPress(int id)', 'id: i32', '()'),
    (3, 'ButtonDepress(int id)', 'id: i32', '()'),
    (4, 'ButtonDownTick(int id)', 'id: i32', '()'),
    (5, 'ButtonMouseEnter(int id)', 'id: i32', '()'),
    (6, 'ButtonMouseLeave(int id)', 'id: i32', '()'),
    (7, 'ButtonMouseMove(int id, int x, int y)', 'id: i32, x: i32, y: i32', '()'),
]

# Sexy::Dialog and subclasses (slots after Widget's 70).
DIALOG = [
    (71, 'SetButtonFont(Font*)', 'font: Ptr', '()'),
    (72, 'SetHeaderFont(Font*)', 'font: Ptr', '()'),
    (73, 'SetLinesFont(Font*)', 'font: Ptr', '()'),
    (74, 'GetPreferredHeight(int theWidth)', 'width: i32', 'i32'),
    (75, 'IsModal()', '', 'bool'),
    (76, 'WaitForResult(bool autoKill)', 'auto_kill: bool', 'i32'),
]

# Sexy::MoneyDialog and subclasses (after Dialog's 76).
MONEY_DIALOG = [
    (77, 'MoneyDialog virtual #77 (set price, disable buttons)', 'arg: i32', '()'),
    (78, 'MoneyDialog virtual #78: CheckboxChecked(int id, bool checked) (default: the click sound)', 'id: i32, checked: bool', '()'),
]

# Sexy::EditWidget (slots after Widget's 70).
EDIT_WIDGET = [
    (71, 'ProcessKey(KeyCode, char)', 'key: i32, c: u8', '()'),
    (72, 'HiliteWord()', '', '()'),
    (73, 'SetFont(Font*, Font* theWidthCheckFont)', 'font: Ptr, width_font: Ptr', '()'),
    (74, 'SetText(const string&, bool leftPosToZero)', 'text: &[u8], left_pos_to_zero: bool', '()'),
    (75, 'IsPartOfWord(char)', 'c: u8', 'bool'),
    (76, 'GetCharAt(int x, int y)', 'x: i32, y: i32', 'i32'),
    (77, 'FocusCursor(bool bigJump)', 'big_jump: bool', '()'),
]

# The EditListener interface (4 slots, declaration order).
EDIT_LISTENER = [
    (1, 'EditWidgetText(int id, const string&)', 'id: i32, text: &[u8]', '()'),
    (2, 'AllowKey(int id, KeyCode)', 'id: i32, key: i32', 'bool'),
    (3, 'AllowChar(int id, char)', 'id: i32, c: u8', 'bool'),
    (4, 'AllowText(int id, const string&)', 'id: i32, text: &[u8]', 'bool'),
]

# Sexy::Slider (slot after Widget's 70).
SLIDER = [
    (71, 'SetValue(double)', 'value: f64', '()'),
]

# Sexy::Checkbox (slots after Widget's 70).
CHECKBOX = [
    (71, 'SetChecked(bool checked, bool tellListener)', 'checked: bool, tell_listener: bool', '()'),
    (72, 'IsChecked()', '', 'bool'),
]

# The SliderListener interface (1 slot).
SLIDER_LISTENER = [
    (1, 'SliderVal(int id, double val)', 'id: i32, val: f64', '()'),
]

# The CheckboxListener interface (1 slot).
CHECKBOX_LISTENER = [
    (1, 'CheckboxChecked(int id, bool checked)', 'id: i32, checked: bool', '()'),
]

# Sexy::ScrollbarWidget (slots after Widget's 70).
SCROLLBAR = [
    (71, 'SetInvisIfNoScroll(bool)', 'invis: bool', '()'),
    (72, 'SetMaxValue(double)', 'value: f64', '()'),
    (73, 'SetPageSize(double)', 'value: f64', '()'),
    (74, 'SetValue(double)', 'value: f64', '()'),
    (75, 'SetHorizontal(bool)', 'horizontal: bool', '()'),
    (76, 'ResizeScrollbar(int x, int y, int w, int h)', 'x: i32, y: i32, w: i32, h: i32', '()'),
    (77, 'AtBottom()', '', 'bool'),
    (78, 'GoToBottom()', '', '()'),
    (79, 'DrawThumb(Graphics*, int x, int y, int w, int h)', 'gfx: &mut Graphics, x: i32, y: i32, w: i32, h: i32', '()'),
    (80, 'GetTrackSize()', '', 'i32'),
    (81, 'GetThumbSize()', '', 'i32'),
    (82, 'GetThumbPosition()', '', 'i32'),
    (83, 'ClampValue()', '', '()'),
    (84, 'SetThumbPosition(int)', 'pos: i32', '()'),
    (85, 'ThumbCompare(int x, int y)', 'x: i32, y: i32', 'i32'),
]

# Sexy::ListWidget (slots after Widget's 70).
LIST_WIDGET = [
    (71, 'GetSortKey(int idx)', 'idx: i32', 'Vec<u8>'),
    (72, 'Sort(bool ascending)', 'ascending: bool', '()'),
    (73, 'GetStringAt(int idx)', 'idx: i32', 'Vec<u8>'),
    (74, 'AddLine(const string&, bool alphabetical)', 'line: &[u8], alphabetical: bool', 'i32'),
    (75, 'SetLine(int idx, const string&)', 'idx: i32, line: &[u8]', '()'),
    (76, 'GetLineCount()', '', 'i32'),
    (77, 'GetLineIdx(const string&)', 'line: &[u8]', 'i32'),
    (78, 'SetColor(const string&, const Color&)', 'line: &[u8], color: Color', '()'),
    (79, 'SetColor(int idx, const Color&)', 'idx: i32, color: Color', '()'),
    (80, 'RemoveLine(int idx)', 'idx: i32', '()'),
    (81, 'RemoveAll()', '', '()'),
    (82, 'GetOptimalWidth()', '', 'i32'),
    (83, 'GetOptimalHeight()', '', 'i32'),
    (84, 'SetSelect(int idx)', 'idx: i32', '()'),
]

# The ScrollListener interface (1 slot).
SCROLL_LISTENER = [
    (1, 'ScrollPosition(int id, double pos)', 'id: i32, pos: f64', '()'),
]

# The ListListener interface (3 slots).
LIST_LISTENER = [
    (1, 'ListClicked(int id, int idx, int clickCount)', 'id: i32, idx: i32, clicks: i32', '()'),
    (2, 'ListClosed(int id)', 'id: i32', '()'),
    (3, 'ListHiliteChanged(int id, int oldIdx, int newIdx)', 'id: i32, old_idx: i32, new_idx: i32', '()'),
]
