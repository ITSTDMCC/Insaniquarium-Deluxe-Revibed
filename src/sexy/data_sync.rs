//! `Sexy::DataReader`, `Sexy::DataWriter` and `Sexy::DataSync`: the binary serializer the
//! save files (`userdata\user%d.dat`, `highscores.dat`, ...) are written with.
//!
//! Byte order and widths are the original's (little-endian, `int` = 4, `short` = 2,
//! `bool`/`byte` = 1, `double` = 8). The original reads either from a `FILE*` or from
//! memory; the port only has the memory mode, and the host loads/stores the file bytes
//! outside of game systems (no blocking I/O inside a system). A read past the end throws
//! `DataReaderException` in the original; here it is an `Err` that callers propagate
//! exactly where the original unwound.

/// The `Sexy::DataReaderException` the original throws on a short read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReadError;

use crate::sexy::types::{Ptr, NULL};
use crate::sexy::g::G;

pub type SyncResult = Result<(), ReadError>;

/// `Sexy::DataReader` in memory mode: +0x8 current pointer, +0xc length, +0x10 position.
#[derive(Clone, Debug, Default)]
pub struct DataReader {
    pub data: Vec<u8>,
    /// +0x10: bytes consumed so far.
    pub pos: usize,
}

impl DataReader {
    /// `DataReader()` + `OpenMemory(bytes, size, false)`.
    pub fn from_bytes(data: Vec<u8>) -> DataReader {
        let mut r = DataReader__005001d0();
        FUN_00502fc0(&mut r, data, false);
        r
    }
}

/// port: 005001d0 Sexy::DataReader::DataReader
/// No file, no memory.
#[allow(non_snake_case)]
pub fn DataReader__005001d0() -> DataReader {
    DataReader { data: Vec::new(), pos: 0 }
}

/// port: 005001f0 FUN_005001f0
/// `DataReader::Close()`: the file (none in the port) and the memory (freed when owned)
/// go; nothing is left to read.
pub fn FUN_005001f0(this: &mut DataReader) {
    this.data = Vec::new();
    this.pos = 0;
}

/// port: 00502fc0 FUN_00502fc0
/// `DataReader::OpenMemory(const void* data, ulong size, bool takeOwnership)`: closes, then
/// reads from the bytes. (A Rust `Vec` is always owned; the flag only decided who frees.)
pub fn FUN_00502fc0(this: &mut DataReader, param_1: Vec<u8>, _param_3: bool) {
    FUN_005001f0(this);
    this.data = param_1;
}

/// port: 00502fb0 Sexy::DataReader::~DataReader
pub fn dtor_DataReader(this: &mut DataReader) {
    FUN_005001f0(this);
}

/// port: 00503fb0 Sexy::DataReader::deleting_destructor
/// The destructor; the memory is the owner's (a Rust value).
pub fn deleting_destructor__00503fb0(this: &mut DataReader, _param_1: u8) {
    dtor_DataReader(this);
}

/// port: 00500270 Sexy::DataReaderException::DataReaderException
/// The exception's copy constructor.
pub fn DataReaderException__00500270(param_1: &ReadError) -> ReadError {
    *param_1
}

/// port: 00502f80 Sexy::DataReaderException::vfunction1
/// The exception's deleting destructor (nothing to free in the port).
pub fn vfunction1(_this: ReadError, _param_1: u8) {}

/// port: 00500290 Sexy::DataReaderException::DataReaderException
/// `DataReader::ReadBytes(void*, ulong)`; the decompiler attached the exception class's
/// name to this address. Throws when fewer than `param_2` bytes remain.
pub fn DataReaderException(this: &mut DataReader, param_1: &mut [u8], param_2: usize) -> SyncResult {
    let end = this.pos + param_2;
    if this.data.len() < end {
        return Err(ReadError);
    }
    param_1[..param_2].copy_from_slice(&this.data[this.pos..end]);
    this.pos = end;
    Ok(())
}

/// port: 00500340 FUN_00500340
/// `DataReader::ReadLong()`.
pub fn FUN_00500340(this: &mut DataReader) -> Result<i32, ReadError> {
    let mut b = [0u8; 4];
    DataReaderException(this, &mut b, 4)?;
    Ok(i32::from_le_bytes(b))
}

/// port: 00500360 FUN_00500360
/// `DataReader::ReadShort()` (zero-extended, as the original masks with 0xffff).
pub fn FUN_00500360(this: &mut DataReader) -> Result<u32, ReadError> {
    let mut b = [0u8; 2];
    DataReaderException(this, &mut b, 2)?;
    Ok(u16::from_le_bytes(b) as u32)
}

/// port: 00500380 FUN_00500380
/// `DataReader::ReadByte()`.
pub fn FUN_00500380(this: &mut DataReader) -> Result<u8, ReadError> {
    let mut b = [0u8; 1];
    DataReaderException(this, &mut b, 1)?;
    Ok(b[0])
}

/// port: 005003a0 FUN_005003a0
/// `DataReader::ReadBool()`.
pub fn FUN_005003a0(this: &mut DataReader) -> Result<bool, ReadError> {
    Ok(FUN_00500380(this)? != 0)
}

/// port: 005003b0 FUN_005003b0
/// `DataReader::ReadDouble()`.
pub fn FUN_005003b0(this: &mut DataReader) -> Result<f64, ReadError> {
    let mut b = [0u8; 8];
    DataReaderException(this, &mut b, 8)?;
    Ok(f64::from_le_bytes(b))
}

/// `Sexy::DataWriter` in memory mode: +0x8 buffer, +0xc length, +0x10 capacity.
#[derive(Clone, Debug, Default)]
pub struct DataWriter {
    pub data: Vec<u8>,
}

/// port: 005003d0 Sexy::DataWriter::DataWriter
/// No file, an empty buffer.
#[allow(non_snake_case)]
pub fn DataWriter__005003d0() -> DataWriter {
    DataWriter { data: Vec::new() }
}

/// port: 005003f0 FUN_005003f0
/// `DataWriter::Close()`: the file (none in the port) closes and the buffer is freed.
pub fn FUN_005003f0(this: &mut DataWriter) {
    this.data = Vec::new();
}

/// port: 00500430 FUN_00500430
/// `DataWriter::EnsureCapacity(ulong n)`: the buffer's capacity doubles until it holds `n`
/// bytes (the old contents kept).
pub fn FUN_00500430(this: &mut DataWriter, param_1: usize) {
    let mut cap = this.data.capacity().max(1);
    if param_1 <= cap {
        return;
    }
    while cap < param_1 {
        cap <<= 1;
    }
    let extra = cap - this.data.len();
    this.data.reserve_exact(extra);
}

/// port: 005030f0 FUN_005030f0
/// `DataWriter::OpenMemory(ulong reserve)`: closes, then writes to a new buffer of at least
/// 32 bytes.
pub fn FUN_005030f0(this: &mut DataWriter, param_1: usize) {
    FUN_005003f0(this);
    let n = if param_1 < 0x20 { 0x20 } else { param_1 };
    this.data = Vec::with_capacity(n);
}

/// port: 005030e0 Sexy::DataWriter::~DataWriter
pub fn dtor_DataWriter(this: &mut DataWriter) {
    FUN_005003f0(this);
}

/// port: 00503fe0 Sexy::DataWriter::deleting_destructor
/// The destructor; the memory is the owner's (a Rust value).
pub fn deleting_destructor__00503fe0(this: &mut DataWriter, _param_1: u8) {
    dtor_DataWriter(this);
}

/// port: 00500480 FUN_00500480
/// `DataWriter::WriteBytes(const void*, ulong)`.
pub fn FUN_00500480(this: &mut DataWriter, param_1: &[u8], param_2: usize) {
    this.data.extend_from_slice(&param_1[..param_2]);
}

/// port: 005004e0 FUN_005004e0
/// `DataWriter::WriteLong(long)`.
pub fn FUN_005004e0(this: &mut DataWriter, param_1: i32) {
    FUN_00500480(this, &param_1.to_le_bytes(), 4);
}

/// port: 00500500 FUN_00500500
/// `DataWriter::WriteShort(short)`.
pub fn FUN_00500500(this: &mut DataWriter, param_1: u16) {
    FUN_00500480(this, &param_1.to_le_bytes(), 2);
}

/// port: 00500520 FUN_00500520
/// `DataWriter::WriteByte(char)`.
pub fn FUN_00500520(this: &mut DataWriter, param_1: u8) {
    FUN_00500480(this, &[param_1], 1);
}

/// port: 00500540 FUN_00500540
/// `DataWriter::WriteBool(bool)`.
pub fn FUN_00500540(this: &mut DataWriter, param_1: bool) {
    FUN_00500520(this, param_1 as u8);
}

/// port: 00500560 FUN_00500560
/// `DataWriter::WriteDouble(double)`.
pub fn FUN_00500560(this: &mut DataWriter, param_1: f64) {
    FUN_00500480(this, &param_1.to_le_bytes(), 8);
}

/// The stream a `DataSync` works on: `mReader` (+0x4) or `mWriter` (+0x8), exactly one
/// is set.
#[derive(Debug)]
pub enum DataIo {
    Read(DataReader),
    Write(DataWriter),
}

/// A pointer field synced with `SyncPointer` (`FUN_005122f0`). The original records the
/// field's address and fills it (or writes its target's id) once everything is synced;
/// the port names the field instead. These are all the pointer fields the game syncs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PtrSlot {
    /// `GameObject +0x98`: the missile on it.
    GameObjectMissle(Ptr),
    /// `Missle +0x1a4`: its target.
    MissleTarget(Ptr),
    /// `Shadow +0x158`: the object it belongs to.
    ShadowOwner(Ptr),
    /// `Coin +0x190`.
    CoinOffset0x3c(Ptr),
}

/// `Sexy::DataSync` (DataSync_data at +0x4).
#[derive(Debug)]
pub struct DataSync {
    /// +0x4 / +0x8 the reader or the writer.
    pub io: DataIo,
    /// +0xc the save version being read or written.
    pub offset_0x8: i32,
    /// +0x10 object -> id (`std::map<void*, int>`); null is id 0.
    pub offset_0x10: std::collections::BTreeMap<Ptr, i32>,
    /// +0x1c id -> object (`std::map<int, void*>`); 0 is null.
    pub offset_0x1c: std::collections::BTreeMap<i32, Ptr>,
    /// +0x28 the pointer fields to resolve at the end (`std::vector<void**>`).
    pub offset_0x28: Vec<PtrSlot>,
    /// +0x38 the next object id.
    pub offset_0x34: i32,
}

fn data_sync(io: DataIo) -> DataSync {
    DataSync { io, offset_0x8: 0, offset_0x10: Default::default(), offset_0x1c: Default::default(), offset_0x28: Vec::new(), offset_0x34: 0 }
}

/// port: 00513e30 Sexy::DataSync::DataSync
/// `DataSync(DataReader&)`.
pub fn DataSync__00513e30(param_1: DataReader) -> DataSync {
    let mut s = data_sync(DataIo::Read(DataReader::default()));
    FUN_00513020(&mut s);
    s.io = DataIo::Read(param_1);
    s
}

/// port: 00513ee0 Sexy::DataSync::DataSync
/// `DataSync(DataWriter&)`.
pub fn DataSync__00513ee0(param_1: DataWriter) -> DataSync {
    let mut s = data_sync(DataIo::Write(DataWriter::default()));
    FUN_00513020(&mut s);
    s.io = DataIo::Write(param_1);
    s
}

/// port: 00513020 FUN_00513020
/// `Reset()`: no stream (here the stream is replaced by the constructor), fresh tables.
pub fn FUN_00513020(this: &mut DataSync) {
    FUN_005121a0(this);
}

/// port: 005121a0 FUN_005121a0
/// `ResetPointerTable()`: empties both maps and the pending fields, then maps null to id 0;
/// ids start at 1.
pub fn FUN_005121a0(this: &mut DataSync) {
    this.offset_0x1c.clear();
    this.offset_0x10.clear();
    this.offset_0x28.clear();
    this.offset_0x34 = 1;
    this.offset_0x10.insert(NULL, 0);
    this.offset_0x1c.insert(0, NULL);
}

/// port: 005120e0 Sexy::DataSync::~DataSync
pub fn dtor_DataSync(this: &mut DataSync) {
    this.offset_0x28.clear();
    this.offset_0x1c.clear();
    this.offset_0x10.clear();
}

/// port: 00512ff0 Sexy::DataSync::deleting_destructor
/// The destructor; the memory is the caller's (a Rust value, dropped by its owner).
pub fn deleting_destructor__00512ff0(this: &mut DataSync, _param_1: u8) {
    dtor_DataSync(this);
}

/// port: 00512270 FUN_00512270
/// `RegisterPointer(void*)`: gives an object the next id the first time it is seen (the
/// writer and the reader number objects in the same order).
pub fn FUN_00512270(this: &mut DataSync, param_1: Ptr) {
    if this.offset_0x10.contains_key(&param_1) {
        return;
    }
    let id = this.offset_0x34;
    this.offset_0x34 = id + 1;
    this.offset_0x10.insert(param_1, id);
    this.offset_0x1c.insert(id, param_1);
}

/// port: 005122f0 FUN_005122f0
/// `SyncPointer(void**)`: remembers the field; it is written or filled in at the end.
pub fn FUN_005122f0(this: &mut DataSync, param_1: PtrSlot) {
    this.offset_0x28.push(param_1);
}

/// port: 00512310 FUN_00512310
/// `ResolvePointers()`: writing, each remembered field's target id (its value now);
/// reading, each field gets the object with the id read (an unknown id throws). Then the
/// tables are reset.
pub fn FUN_00512310(g: &mut G, this: &mut DataSync) -> SyncResult {
    let slots = this.offset_0x28.clone();
    if let DataIo::Write(w) = &mut this.io {
        for s in slots {
            let p = crate::game::game_object::ptr_slot_get(g, s);
            let id = *this.offset_0x10.get(&p).expect("SyncPointer target was never registered");
            FUN_005004e0(w, id);
        }
    } else {
        for s in slots {
            let DataIo::Read(r) = &mut this.io else { unreachable!() };
            let id = FUN_00500340(r)?;
            let Some(&p) = this.offset_0x1c.get(&id) else { return Err(ReadError) };
            crate::game::game_object::ptr_slot_set(g, s, p);
        }
    }
    FUN_005121a0(this);
    Ok(())
}

/// port: 00503010 FUN_00503010
/// `DataSync::SyncLong(int&)`.
pub fn FUN_00503010(this: &mut DataSync, param_1: &mut i32) -> SyncResult {
    match &mut this.io {
        crate::sexy::data_sync::DataIo::Read(r) => *param_1 = FUN_00500340(r)?,
        crate::sexy::data_sync::DataIo::Write(w) => FUN_005004e0(w, *param_1),
    }
    Ok(())
}

/// port: 00503040 FUN_00503040
/// `DataSync::SyncByte(int&)`: one byte on disk, zero-extended into an int.
pub fn FUN_00503040(this: &mut DataSync, param_1: &mut i32) -> SyncResult {
    match &mut this.io {
        crate::sexy::data_sync::DataIo::Read(r) => *param_1 = FUN_00500380(r)? as i32,
        crate::sexy::data_sync::DataIo::Write(w) => FUN_00500520(w, *param_1 as u8),
    }
    Ok(())
}

/// port: 00503070 FUN_00503070
/// `DataSync::SyncBool(bool&)`.
pub fn FUN_00503070(this: &mut DataSync, param_1: &mut bool) -> SyncResult {
    match &mut this.io {
        crate::sexy::data_sync::DataIo::Read(r) => *param_1 = FUN_005003a0(r)?,
        crate::sexy::data_sync::DataIo::Write(w) => FUN_00500540(w, *param_1),
    }
    Ok(())
}

/// port: 005030a0 FUN_005030a0
/// `DataSync::SyncDouble(double&)`.
pub fn FUN_005030a0(this: &mut DataSync, param_1: &mut f64) -> SyncResult {
    match &mut this.io {
        crate::sexy::data_sync::DataIo::Read(r) => *param_1 = FUN_005003b0(r)?,
        crate::sexy::data_sync::DataIo::Write(w) => FUN_00500560(w, *param_1),
    }
    Ok(())
}

/// port: 00502ff0 FUN_00502ff0
/// `DataSync::SyncBytes(void*, ulong)`.
pub fn FUN_00502ff0(this: &mut DataSync, param_1: &mut [u8], param_2: usize) -> SyncResult {
    match &mut this.io {
        crate::sexy::data_sync::DataIo::Read(r) => DataReaderException(r, param_1, param_2)?,
        crate::sexy::data_sync::DataIo::Write(w) => FUN_00500480(w, param_1, param_2),
    }
    Ok(())
}

/// port: 005057f0 FUN_005057f0
/// `DataReader::ReadString(std::string&)`: a 16-bit length, then the bytes.
pub fn FUN_005057f0(this: &mut DataReader, param_1: &mut Vec<u8>) -> SyncResult {
    let n = (FUN_00500360(this)? & 0xffff) as usize;
    param_1.resize(n, 0);
    DataReaderException(this, param_1, n)
}

/// port: 00504010 FUN_00504010
/// `DataWriter::WriteString(const std::string&)`: the length as a short, then the bytes.
pub fn FUN_00504010(this: &mut DataWriter, param_1: &[u8]) {
    FUN_00500500(this, param_1.len() as u16);
    FUN_00500480(this, param_1, param_1.len() & 0xffff);
}

/// port: 00505860 FUN_00505860
/// `DataSync::SyncString(std::string&)`. Strings are byte strings (the game's code page),
/// kept as `Vec<u8>` so round trips are exact.
pub fn FUN_00505860(this: &mut DataSync, param_1: &mut Vec<u8>) -> SyncResult {
    match &mut this.io {
        crate::sexy::data_sync::DataIo::Read(r) => FUN_005057f0(r, param_1),
        crate::sexy::data_sync::DataIo::Write(w) => {
            FUN_00504010(w, param_1);
            Ok(())
        }
    }
}

/// port: 00500230 Sexy::DataReaderException::DataReaderException
/// `DataReader::Rewind(ulong n)` (the decompiler attached the exception class's name):
/// steps back `n` bytes; throws when fewer than `n` have been read.
pub fn DataReaderException__00500230(this: &mut DataReader, param_1: usize) -> SyncResult {
    if this.pos < param_1 {
        return Err(ReadError);
    }
    this.pos -= param_1;
    Ok(())
}
