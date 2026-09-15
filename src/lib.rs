use core::any::Any;

use std::fs::File;
use std::io;

use io::Write;

use arrow_schema::DataType;
use arrow_schema::Schema;
use arrow_schema::SchemaRef;

use arrow_array::RecordBatch;
use arrow_array::RecordBatchReader;

use arrow_array::array::Array;
use arrow_array::array::ArrayRef;

use arrow_array::array::BinaryArray;
use arrow_array::array::BinaryViewArray;
use arrow_array::array::BooleanArray;
use arrow_array::array::Date32Array;
use arrow_array::array::Date64Array;
use arrow_array::array::Decimal32Array;
use arrow_array::array::Decimal64Array;
use arrow_array::array::Decimal128Array;
use arrow_array::array::Decimal256Array;
use arrow_array::array::DurationMicrosecondArray;
use arrow_array::array::DurationMillisecondArray;
use arrow_array::array::DurationNanosecondArray;
use arrow_array::array::DurationSecondArray;
use arrow_array::array::Float16Array;
use arrow_array::array::Float32Array;
use arrow_array::array::Float64Array;
use arrow_array::array::Int8Array;
use arrow_array::array::Int8DictionaryArray;
use arrow_array::array::Int16Array;
use arrow_array::array::Int16DictionaryArray;
use arrow_array::array::Int16RunArray;
use arrow_array::array::Int32Array;
use arrow_array::array::Int32DictionaryArray;
use arrow_array::array::Int32RunArray;
use arrow_array::array::Int64Array;
use arrow_array::array::Int64DictionaryArray;
use arrow_array::array::Int64RunArray;
use arrow_array::array::IntervalDayTimeArray;
use arrow_array::array::IntervalMonthDayNanoArray;
use arrow_array::array::IntervalYearMonthArray;
use arrow_array::array::LargeBinaryArray;
use arrow_array::array::LargeListArray;
use arrow_array::array::LargeListViewArray;
use arrow_array::array::LargeStringArray;
use arrow_array::array::ListArray;
use arrow_array::array::ListViewArray;
use arrow_array::array::StringArray;
use arrow_array::array::StringViewArray;
use arrow_array::array::Time32MillisecondArray;
use arrow_array::array::Time32SecondArray;
use arrow_array::array::Time64MicrosecondArray;
use arrow_array::array::Time64NanosecondArray;
use arrow_array::array::TimestampMicrosecondArray;
use arrow_array::array::TimestampMillisecondArray;
use arrow_array::array::TimestampNanosecondArray;
use arrow_array::array::TimestampSecondArray;
use arrow_array::array::UInt8Array;
use arrow_array::array::UInt8DictionaryArray;
use arrow_array::array::UInt16Array;
use arrow_array::array::UInt16DictionaryArray;
use arrow_array::array::UInt32Array;
use arrow_array::array::UInt32DictionaryArray;
use arrow_array::array::UInt64Array;
use arrow_array::array::UInt64DictionaryArray;

use parquet::arrow::arrow_reader::ParquetRecordBatchReader;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

pub struct Batch(pub RecordBatch);

pub fn array2array_type_name(arr: &dyn Array) -> Option<&'static str> {
    let aany: &dyn Any = arr.as_any();

    if aany.downcast_ref::<BinaryArray>().is_some() {
        return Some("BinaryArray");
    }
    if aany.downcast_ref::<BinaryViewArray>().is_some() {
        return Some("BinaryViewArray");
    }
    if aany.downcast_ref::<BooleanArray>().is_some() {
        return Some("BooleanArray");
    }
    if aany.downcast_ref::<Date32Array>().is_some() {
        return Some("Date32Array");
    }
    if aany.downcast_ref::<Date64Array>().is_some() {
        return Some("Date64Array");
    }
    if aany.downcast_ref::<Decimal32Array>().is_some() {
        return Some("Decimal32Array");
    }
    if aany.downcast_ref::<Decimal64Array>().is_some() {
        return Some("Decimal64Array");
    }
    if aany.downcast_ref::<Decimal128Array>().is_some() {
        return Some("Decimal128Array");
    }
    if aany.downcast_ref::<Decimal256Array>().is_some() {
        return Some("Decimal256Array");
    }
    if aany.downcast_ref::<DurationMicrosecondArray>().is_some() {
        return Some("DurationMicrosecondArray");
    }
    if aany.downcast_ref::<DurationMillisecondArray>().is_some() {
        return Some("DurationMillisecondArray");
    }
    if aany.downcast_ref::<DurationNanosecondArray>().is_some() {
        return Some("DurationNanosecondArray");
    }
    if aany.downcast_ref::<DurationSecondArray>().is_some() {
        return Some("DurationSecondArray");
    }
    if aany.downcast_ref::<Float16Array>().is_some() {
        return Some("Float16Array");
    }
    if aany.downcast_ref::<Float32Array>().is_some() {
        return Some("Float32Array");
    }
    if aany.downcast_ref::<Float64Array>().is_some() {
        return Some("Float64Array");
    }
    if aany.downcast_ref::<Int8Array>().is_some() {
        return Some("Int8Array");
    }
    if aany.downcast_ref::<Int8DictionaryArray>().is_some() {
        return Some("Int8DictionaryArray");
    }
    if aany.downcast_ref::<Int16Array>().is_some() {
        return Some("Int16Array");
    }
    if aany.downcast_ref::<Int16DictionaryArray>().is_some() {
        return Some("Int16DictionaryArray");
    }
    if aany.downcast_ref::<Int16RunArray>().is_some() {
        return Some("Int16RunArray");
    }
    if aany.downcast_ref::<Int32Array>().is_some() {
        return Some("Int32Array");
    }
    if aany.downcast_ref::<Int32DictionaryArray>().is_some() {
        return Some("Int32DictionaryArray");
    }
    if aany.downcast_ref::<Int32RunArray>().is_some() {
        return Some("Int32RunArray");
    }
    if aany.downcast_ref::<Int64Array>().is_some() {
        return Some("Int64Array");
    }
    if aany.downcast_ref::<Int64DictionaryArray>().is_some() {
        return Some("Int64DictionaryArray");
    }
    if aany.downcast_ref::<Int64RunArray>().is_some() {
        return Some("Int64RunArray");
    }
    if aany.downcast_ref::<IntervalDayTimeArray>().is_some() {
        return Some("IntervalDayTimeArray");
    }
    if aany.downcast_ref::<IntervalMonthDayNanoArray>().is_some() {
        return Some("IntervalMonthDayNanoArray");
    }
    if aany.downcast_ref::<IntervalYearMonthArray>().is_some() {
        return Some("IntervalYearMonthArray");
    }
    if aany.downcast_ref::<LargeBinaryArray>().is_some() {
        return Some("LargeBinaryArray");
    }
    if aany.downcast_ref::<LargeListArray>().is_some() {
        return Some("LargeListArray");
    }
    if aany.downcast_ref::<LargeListViewArray>().is_some() {
        return Some("LargeListViewArray");
    }
    if aany.downcast_ref::<LargeStringArray>().is_some() {
        return Some("LargeStringArray");
    }
    if aany.downcast_ref::<ListArray>().is_some() {
        return Some("ListArray");
    }
    if aany.downcast_ref::<ListViewArray>().is_some() {
        return Some("ListViewArray");
    }
    if aany.downcast_ref::<StringArray>().is_some() {
        return Some("StringArray");
    }
    if aany.downcast_ref::<StringViewArray>().is_some() {
        return Some("StringViewArray");
    }
    if aany.downcast_ref::<Time32MillisecondArray>().is_some() {
        return Some("Time32MillisecondArray");
    }
    if aany.downcast_ref::<Time32SecondArray>().is_some() {
        return Some("Time32SecondArray");
    }
    if aany.downcast_ref::<Time64MicrosecondArray>().is_some() {
        return Some("Time64MicrosecondArray");
    }
    if aany.downcast_ref::<Time64NanosecondArray>().is_some() {
        return Some("Time64NanosecondArray");
    }
    if aany.downcast_ref::<TimestampMicrosecondArray>().is_some() {
        return Some("TimestampMicrosecondArray");
    }
    if aany.downcast_ref::<TimestampMillisecondArray>().is_some() {
        return Some("TimestampMillisecondArray");
    }
    if aany.downcast_ref::<TimestampNanosecondArray>().is_some() {
        return Some("TimestampNanosecondArray");
    }
    if aany.downcast_ref::<TimestampSecondArray>().is_some() {
        return Some("TimestampSecondArray");
    }
    if aany.downcast_ref::<UInt8Array>().is_some() {
        return Some("UInt8Array");
    }
    if aany.downcast_ref::<UInt8DictionaryArray>().is_some() {
        return Some("UInt8DictionaryArray");
    }
    if aany.downcast_ref::<UInt16Array>().is_some() {
        return Some("UInt16Array");
    }
    if aany.downcast_ref::<UInt16DictionaryArray>().is_some() {
        return Some("UInt16DictionaryArray");
    }
    if aany.downcast_ref::<UInt32Array>().is_some() {
        return Some("UInt32Array");
    }
    if aany.downcast_ref::<UInt32DictionaryArray>().is_some() {
        return Some("UInt32DictionaryArray");
    }
    if aany.downcast_ref::<UInt64Array>().is_some() {
        return Some("UInt64Array");
    }
    if aany.downcast_ref::<UInt64DictionaryArray>().is_some() {
        return Some("UInt64DictionaryArray");
    }

    Some("GenericBinaryArray or GenericStringArray")
}

impl Batch {
    pub fn cols2dtsink<S>(&self, sink: &mut S) -> Result<(), io::Error>
    where
        S: FnMut(&DataType) -> Result<(), io::Error>,
    {
        let cols: &[ArrayRef] = self.0.columns();
        for col in cols {
            let a: &dyn Array = &col;
            let dtyp: &DataType = a.data_type();
            sink(dtyp)?;
        }
        Ok(())
    }
}

impl Batch {
    pub fn cols2atypsink<S>(&self, sink: &mut S) -> Result<(), io::Error>
    where
        S: FnMut(&str) -> Result<(), io::Error>,
    {
        let cols: &[ArrayRef] = self.0.columns();
        for col in cols {
            let a: &dyn Array = &col;
            let atyp: &str = array2array_type_name(a).unwrap_or_default();
            sink(atyp)?;
        }
        Ok(())
    }
}

pub fn wtr2dtypsink_json<W>(mut wtr: W) -> impl FnMut(&DataType) -> Result<(), io::Error>
where
    W: Write,
{
    move |dtyp: &DataType| {
        serde_json::to_writer(&mut wtr, dtyp)?;
        writeln!(&mut wtr)
    }
}

pub struct SchRef(pub SchemaRef);

impl SchRef {
    pub fn as_schema(&self) -> &Schema {
        &self.0
    }
}

impl SchRef {
    pub fn to_json_to_writer<W>(&self, wtr: &mut W) -> Result<(), io::Error>
    where
        W: Write,
    {
        let sch: &Schema = self.as_schema();
        serde_json::to_writer(wtr, sch).map_err(io::Error::other)
    }
}

pub fn wtr2schema_sink_json<W>(mut wtr: W) -> impl FnMut(SchemaRef) -> Result<(), io::Error>
where
    W: Write,
{
    move |sref: SchemaRef| SchRef(sref).to_json_to_writer(&mut wtr)
}

pub fn sink_json_stdout() -> impl FnMut(SchemaRef) -> Result<(), io::Error> {
    wtr2schema_sink_json(io::stdout().lock())
}

pub fn rbr2sref<R>(rbr: &R) -> SchemaRef
where
    R: RecordBatchReader,
{
    rbr.schema()
}

pub struct PqReader(pub ParquetRecordBatchReader);

impl PqReader {
    pub fn schema(&self) -> SchemaRef {
        rbr2sref(&self.0)
    }
}

impl PqReader {
    pub fn into_batch(mut self) -> Result<RecordBatch, io::Error> {
        self.0
            .next()
            .and_then(|rslt| rslt.ok())
            .ok_or(io::Error::other("unable to get the batch"))
    }
}

pub struct PqFile(pub File);

impl PqFile {
    pub fn into_builder(self) -> Result<ParquetRecordBatchReaderBuilder<File>, io::Error> {
        ParquetRecordBatchReaderBuilder::try_new(self.0).map_err(io::Error::other)
    }
}

impl PqFile {
    pub fn into_reader(self) -> Result<ParquetRecordBatchReader, io::Error> {
        self.into_builder()
            .and_then(|bldr| bldr.build().map_err(io::Error::other))
    }
}

impl PqFile {
    pub fn into_schema(self) -> Result<SchemaRef, io::Error> {
        self.into_reader().map(|rdr| PqReader(rdr).schema())
    }
}

impl PqFile {
    pub fn into_batch(self) -> Result<RecordBatch, io::Error> {
        self.into_reader()
            .and_then(|rdr| PqReader(rdr).into_batch())
    }
}

pub fn filename2sch_source(filename: String) -> impl Fn() -> Result<SchemaRef, io::Error> {
    move || {
        let f: File = File::open(&filename)?;
        PqFile(f).into_schema()
    }
}

pub fn filename2bat_source(filename: String) -> impl Fn() -> Result<RecordBatch, io::Error> {
    move || {
        let f: File = File::open(&filename)?;
        PqFile(f).into_batch()
    }
}

pub struct PqFilename(pub String);

impl PqFilename {
    pub fn into_sink<S>(self, mut sink: S) -> Result<(), io::Error>
    where
        S: FnMut(SchemaRef) -> Result<(), io::Error>,
    {
        let src = filename2sch_source(self.0);
        let sch: SchemaRef = src()?;
        sink(sch)
    }
}

impl PqFilename {
    pub fn into_typ_sink<S>(self, mut sink: S) -> Result<(), io::Error>
    where
        S: FnMut(&DataType) -> Result<(), io::Error>,
    {
        let src = filename2bat_source(self.0);
        let bat: RecordBatch = src()?;
        Batch(bat).cols2dtsink(&mut sink)
    }
}

impl PqFilename {
    pub fn into_atyp_sink<S>(self, mut sink: S) -> Result<(), io::Error>
    where
        S: FnMut(&str) -> Result<(), io::Error>,
    {
        let src = filename2bat_source(self.0);
        let bat: RecordBatch = src()?;
        Batch(bat).cols2atypsink(&mut sink)
    }
}

impl PqFilename {
    pub fn into_sink_default(self) -> Result<(), io::Error> {
        self.into_sink(sink_json_stdout())
    }
}

impl PqFilename {
    pub fn into_typ_sink_default(self) -> Result<(), io::Error> {
        self.into_typ_sink(wtr2dtypsink_json(io::stdout().lock()))
    }
}

impl PqFilename {
    pub fn into_atyp_sink_default(self) -> Result<(), io::Error> {
        self.into_atyp_sink(|atyp: &str| {
            println!("{atyp}");
            Ok(())
        })
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Mode {
    #[default]
    ArrayType,

    Detailed,

    TypeOnly,
}

impl std::str::FromStr for Mode {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "array" => Ok(Self::ArrayType),
            "detailed" => Ok(Self::Detailed),
            "typeonly" => Ok(Self::TypeOnly),
            _ => Err(io::Error::other(format!("unknown type: {s}"))),
        }
    }
}

impl Mode {
    pub fn show_type_info(self, f: PqFilename) -> Result<(), io::Error> {
        match self {
            Self::Detailed => f.into_sink_default(),
            Self::TypeOnly => f.into_typ_sink_default(),
            Self::ArrayType => f.into_atyp_sink_default(),
        }
    }
}
