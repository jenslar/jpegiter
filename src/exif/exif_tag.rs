/// Exif tags, courtesy of https://exiftool.org/TagNames/EXIF.html.
pub enum ExifTag {
    Acceleration = 0x9404,
    ActiveArea = 0xC68D,
    AdventRevision = 0x8336,
    AdventScale = 0x8335,
    AfcpIptc = 0x8568,
    AffineTransformMat = 0x80BC,
    AliasLayerMetadata = 0xC660,
    AlphaByteCount = 0xBCC3,
    AlphaDataDiscard = 0xBCC5,
    AlphaOffset = 0xBCC2,
    AmbientTemperature = 0x9400,
    AnalogBalance = 0xC627,
    Annotations = 0xC44F,
    AntiAliasStrength = 0xC632,
    ApertureValue = 0x9202,
    ApplicationNotes = 0x02BC,
    Artist = 0x013B,
    AsShotICCProfile = 0xC68F,
    AsShotNeutral = 0xC628,
    AsShotPreProfileMatrix = 0xC690,
    AsShotProfileName = 0xC6F6,
    AsShotWhiteXY = 0xC629,
    BackgroundColorIndicator = 0x84E8,
    BackgroundColorValue = 0x84EA,
    BadFaxLines = 0x0146,
    BaselineExposure = 0xC62A,
    BaselineExposureOffset = 0xC7A5,
    BaselineNoise = 0xC62B,
    BaselineSharpness = 0xC62C,
    BatteryLevel = 0x828F,
    BayerGreenSplit = 0xC62D,
    BestQualityScale = 0xC65C,
    BitsPerExtendedRunLength = 0x84E5,
    BitsPerRunLength = 0x84E4,
    BitsPerSample = 0x0102,
    BlackLevel = 0xC61A,
    BlackLevelDeltaH = 0xC61B,
    BlackLevelDeltaV = 0xC61C,
    BlackLevelRepeatDim = 0xC619,
    Brightness = 0xFE53,
    BrightnessValue = 0x9203,
    CacheVersion = 0xC7AA,
    CalibrationIlluminant1 = 0xC65A,
    CalibrationIlluminant2 = 0xC65B,
    CameraCalibration1 = 0xC623,
    CameraCalibration2 = 0xC624,
    CameraCalibrationSig = 0xC6F3,
    CameraElevationAngle = 0x9405,
    CameraLabel = 0xC7A1,
    CameraSerialNumber = 0xC62F,
    CellLength = 0x0109,
    CellWidth = 0x0108,
    CFALayout = 0xC617,
    CFAPattern = 0xA302,
    CFAPattern2 = 0x828E,
    CFAPlaneColor = 0xC616,
    CFARepeatPatternDim = 0x828D,
    ChromaBlurRadius = 0xC631,
    ChromaticAberrationCorrection = 0x7034,
    ChromaticAberrationCorrParams = 0x7035,
    CIP3DataFile = 0x923A,
    CIP3Sheet = 0x923B,
    CIP3Side = 0x923C,
    CleanFaxData = 0x0147,
    ClipPath = 0x0157,
    CMYKEquivalent = 0x84F0,
    CodingMethods = 0x0193,
    ColorCharacterization = 0x84ED,
    ColorimetricReference = 0xC6BF,
    ColorMap = 0x0140,
    ColorMatrix1 = 0xC621,
    ColorMatrix2 = 0xC622,
    ColorResponseUnit = 0x012C,
    ColorSequence = 0x84E1,
    ColorSpace = 0xA001,
    ColorTable = 0x84E6,
    ComponentsConfiguration = 0x9101,
    CompositeImage = 0xA460,
    CompositeImageCount = 0xA461,
    CompositeImageExposureTimes = 0xA462,
    CompressedBitsPerPixel = 0x9102,
    Compression = 0x0103,
    ConsecutiveBadFaxLines = 0x0148,
    Contrast1 = 0xA408,
    Contrast2 = 0xFE54,
    Converter = 0xFE4D,
    Copyright = 0x8298,
    CR2CFAPattern = 0xC5E0,
    CreateDate = 0x9004,
    CurrentICCProfile = 0xC691,
    CurrentPreProfileMatrix = 0xC692,
    CustomRendered = 0xA401,
    DataType = 0x80E4,
    DateTimeOriginal = 0x9003,
    Decode = 0x01B1,
    DefaultBlackRender = 0xC7A6,
    DefaultCropOrigin = 0xC61F,
    DefaultCropSize = 0xC620,
    DefaultImageColor = 0x01B2,
    DefaultScale = 0xC61E,
    DefaultUserCrop = 0xC7B5,
    DepthFar = 0xC7EB,
    DepthFormat = 0xC7E9,
    DepthMeasureType = 0xC7ED,
    DepthNear = 0xC7EA,
    DepthUnits = 0xC7EC,
    DeviceSettingDescription = 0xA40B,
    DigitalZoomRatio = 0xA404,
    DistortionCorrection = 0x7036,
    DistortionCorrParams = 0x7037,
    DNGBackwardVersion = 0xC613,
    DNGLensInfo = 0xC630,
    DNGVersion = 0xC612,
    DocumentName = 0x010D,
    DotRange = 0x0150,
    EnhanceParams = 0xC7EE,
    ExifImageHeight = 0xA003,
    ExifImageWidth = 0xA002,
    ExifOffset = 0x8769,
    ExifVersion = 0x9000,
    ExpandFilm = 0xAFC2,
    ExpandFilterLens = 0xAFC3,
    ExpandFlashLamp = 0xAFC5,
    ExpandLens = 0xAFC1,
    ExpandScanner = 0xAFC4,
    ExpandSoftware = 0xAFC0,
    Exposure = 0xFE51,
    ExposureCompensation = 0x9204,
    ExposureIndex1 = 0x9215,
    ExposureIndex2 = 0xA215,
    ExposureMode = 0xA402,
    ExposureProgram = 0x8822,
    ExposureTime = 0x829A,
    ExtraSamples = 0x0152,
    FaxProfile = 0x0192,
    FaxRecvParams = 0x885C,
    FaxRecvTime = 0x885E,
    FaxSubAddress = 0x885D,
    FedexEDR = 0x8871,
    FileSource = 0xA300,
    FillOrder = 0x010A,
    Flash = 0x9209,
    FlashEnergy1 = 0x920B,
    FlashEnergy2 = 0xA20B,
    FlashpixVersion = 0xA000,
    FNumber = 0x829D,
    FocalLength = 0x920A,
    FocalLengthIn35mmFormat = 0xA405,
    FocalPlaneResolutionUnit1 = 0x9210,
    FocalPlaneResolutionUnit2 = 0xA210,
    FocalPlaneXResolution1 = 0x920E,
    FocalPlaneXResolution2 = 0xA20E,
    FocalPlaneYResolution1 = 0x920F,
    FocalPlaneYResolution2 = 0xA20F,
    ForwardMatrix1 = 0xC714,
    ForwardMatrix2 = 0xC715,
    FovCot = 0x8218,
    FrameRate = 0xC764,
    FreeByteCounts = 0x0121,
    FreeOffsets = 0x0120,
    GainControl = 0xA407,
    Gamma = 0xA500,
    GDALMetadata = 0xA480,
    GDALNoData = 0xA481,
    GeoTiffAsciiParams = 0x87B1,
    GeoTiffDirectory = 0x87AF,
    GeoTiffDoubleParams = 0x87B0,
    GlobalParametersIFD = 0x0190,
    GooglePlusUploadCode = 0x9009,
    GPSInfo = 0x8825,
    GrayResponseCurve = 0x0123,
    GrayResponseUnit = 0x0122,
    HalftoneHints = 0x0141,
    HasselbladExif = 0xC51B,
    HasselbladRawImage = 0xB4C3,
    HCUsage = 0x84EE,
    HeightResolution = 0xBC83,
    HostComputer = 0x013C,
    Humidity = 0x9401,
    IccProfile = 0x8773,
    ImageByteCount = 0xBCC1,
    ImageColorIndicator = 0x84E7,
    ImageColorValue = 0x84E9,
    ImageDataDiscard = 0xBCC4,
    ImageDepth = 0x80E5,
    ImageDescription = 0x010E,
    ImageFullHeight = 0x8215,
    ImageFullWidth = 0x8214,
    ImageHeight1 = 0x0101,
    ImageHeight2 = 0xBC81,
    ImageHistory1 = 0x9213,
    ImageHistory2 = 0xA213,
    ImageID = 0x800D,
    ImageLayer = 0x87AC,
    ImageNumber1 = 0x9211,
    ImageNumber2 = 0xA211,
    ImageOffset = 0xBCC0,
    ImageReferencePoints = 0x80B9,
    ImageSourceData = 0x935C,
    ImageType = 0xBC04,
    ImageUniqueID = 0xA420,
    ImageWidth1 = 0x0100,
    ImageWidth2 = 0xBC80,
    Indexed = 0x015A,
    INGRReserved = 0x8481,
    InkNames = 0x014D,
    InkSet = 0x014C,
    IntergraphFlagRegisters = 0x847F,
    IntergraphMatrix = 0x8480,
    IntergraphPacketData = 0x847E,
    Interlace = 0x8829,
    InteropIndex = 0x0001,
    InteropOffset = 0xA005,
    InteropVersion = 0x0002,
    IPTC = 0x83BB,
    ISO = 0x8827,
    ISOSpeed = 0x8833,
    ISOSpeedLatitudeyyy = 0x8834,
    ISOSpeedLatitudezzz = 0x8835,
    IT8Header = 0x84E2,
    JBIGOptions = 0x87BE,
    JPEGACTables = 0x0209,
    JPEGDCTables = 0x0208,
    JPEGLosslessPredictors = 0x0205,
    JPEGPointTransforms = 0x0206,
    JPEGProc = 0x0200,
    JPEGQTables = 0x0207,
    JPEGRestartInterval = 0x0203,
    JPEGTables1 = 0x015B,
    JPEGTables2 = 0x01B5,
    JPLCartoIFD = 0x85D7,
    KdcIfd = 0xFE00,
    KodakIFD = 0x8290,
    LeafData = 0x8606,
    LeafSubIFD = 0x888A,
    Lens = 0xFDEA,
    LensInfo = 0xA432,
    LensMake = 0xA433,
    LensModel = 0xA434,
    LensSerialNumber = 0xA435,
    LightSource = 0x9208,
    LinearizationTable = 0xC618,
    LinearResponseLimit = 0xC62E,
    LocalizedCameraModel = 0xC615,
    Make = 0x010F,
    MakerNoteApple = 0x927C,
    MakerNoteSafety = 0xC635,
    MaskedAreas = 0xC68E,
    MatrixWorldToCamera = 0x821A,
    MatrixWorldToScreen = 0x8219,
    Matteing = 0x80E3,
    MaxApertureValue = 0x9205,
    MaxSampleValue = 0x0119,
    MDColorTable = 0x82A7,
    MDFileTag = 0x82A5,
    MDFileUnits = 0x82AC,
    MDLabName = 0x82A8,
    MDPrepDate = 0x82AA,
    MDPrepTime = 0x82AB,
    MDSampleInfo = 0x82A9,
    MDScalePixel = 0x82A6,
    MeteringMode = 0x9207,
    MinSampleValue = 0x0118,
    Model = 0x0110,
    Model2 = 0x827D,
    ModelTiePoint = 0x8482,
    ModelTransform = 0x85D8,
    ModeNumber = 0x0195,
    ModifyDate = 0x0132,
    MoireFilter = 0xFE58,
    MSDocumentText = 0x932F,
    MSDocumentTextPosition = 0x9331,
    MSPropertySetStorage = 0x9330,
    MultiProfiles = 0x8780,
    NewRawImageDigest = 0xC7A7,
    NikonNEFInfo = 0xC7D5,
    Noise1 = 0x920D,
    Noise2 = 0xA20D,
    NoiseProfile = 0xC761,
    NoiseReductionApplied = 0xC6F7,
    NumberofInks = 0x014E,
    OceApplicationSelector = 0xC428,
    OceIDNumber = 0xC429,
    OceImageLogic = 0xC42A,
    OceScanjobDesc = 0xC427,
    OffsetSchema = 0xEA1D,
    OffsetTime = 0x9010,
    OffsetTimeDigitized = 0x9012,
    OffsetTimeOriginal = 0x9011,
    OldSubfileType = 0x00FF,
    OpcodeList1 = 0xC740,
    OpcodeList2 = 0xC741,
    OpcodeList3 = 0xC74E,
    OPIProxy = 0x015F,
    Opto = 0x8828,
    Orientation = 0x0112,
    OriginalBestQualitySize = 0xC792,
    OriginalDefaultCropSize = 0xC793,
    OriginalDefaultFinalSize = 0xC791,
    OriginalFileName = 0xC573,
    OriginalRawFileData = 0xC68C,
    OriginalRawFileDigest = 0xC71D,
    OriginalRawFileName = 0xC68B,
    OwnerName1 = 0xA430,
    OwnerName2 = 0xFDE8,
    Padding = 0xEA1C,
    PageName = 0x011D,
    PageNumber = 0x0129,
    PanasonicTitle = 0xC6D2,
    PanasonicTitle2 = 0xC6D3,
    PhotometricInterpretation = 0x0106,
    PhotoshopSettings = 0x8649,
    PixelFormat = 0xBC01,
    PixelIntensityRange = 0x84EB,
    PixelMagicJBIGOptions = 0x85B8,
    PixelScale = 0x830E,
    PlanarConfiguration = 0x011C,
    Predictor = 0x013D,
    Pressure = 0x9402,
    PreviewApplicationName = 0xC716,
    PreviewApplicationVersion = 0xC717,
    PreviewColorSpace = 0xC71A,
    PreviewDateTime = 0xC71B,
    PreviewSettingsDigest = 0xC719,
    PreviewSettingsName = 0xC718,
    PrimaryChromaticities = 0x013F,
    PrintIM = 0xC4A5,
    ProcessingSoftware = 0x000B,
    ProfileCalibrationSig = 0xC6F4,
    ProfileCopyright = 0xC6FE,
    ProfileEmbedPolicy = 0xC6FD,
    ProfileHueSatMapData1 = 0xC6FA,
    ProfileHueSatMapData2 = 0xC6FB,
    ProfileHueSatMapDims = 0xC6F9,
    ProfileHueSatMapEncoding = 0xC7A3,
    ProfileIFD = 0xC6F5,
    ProfileLookTableData = 0xC726,
    ProfileLookTableDims = 0xC725,
    ProfileLookTableEncoding = 0xC7A4,
    ProfileName = 0xC6F8,
    ProfileToneCurve = 0xC6FC,
    ProfileType = 0x0191,
    RasterPadding = 0x84E3,
    Rating = 0x4746,
    RatingPercent = 0x4749,
    RawDataUniqueID = 0xC65D,
    RawFile = 0xFE4C,
    RawImageDigest = 0xC71C,
    RawImageSegmentation = 0xC640,
    RawToPreviewGain = 0xC7A8,
    RecommendedExposureIndex = 0x8832,
    ReductionMatrix1 = 0xC625,
    ReductionMatrix2 = 0xC626,
    ReelName = 0xC789,
    ReferenceBlackWhite = 0x0214,
    RegionXformTackPoint = 0x80BA,
    RelatedImageFileFormat = 0x1000,
    RelatedImageHeight = 0x1002,
    RelatedImageWidth = 0x1001,
    RelatedSoundFile = 0xA004,
    ResolutionUnit = 0x0128,
    RowInterleaveFactor = 0xC71F,
    RowsPerStrip = 0x0116,
    SampleFormat = 0x0153,
    SamplesPerPixel = 0x0115,
    SamsungRawByteOrder = 0xA101,
    SamsungRawPointersLength = 0xA011,
    SamsungRawPointersOffset = 0xA010,
    SamsungRawUnknown = 0xA102,
    Saturation1 = 0xA409,
    Saturation2 = 0xFE55,
    SceneCaptureType = 0xA406,
    SceneType = 0xA301,
    SecurityClassification1 = 0x9212,
    SecurityClassification2 = 0xA212,
    SelfTimerMode = 0x882B,
    SEMInfo = 0x8546,
    SensingMethod1 = 0x9217,
    SensingMethod2 = 0xA217,
    SensitivityType = 0x8830,
    SerialNumber1 = 0xA431,
    SerialNumber2 = 0xFDE9,
    Shadows = 0xFE52,
    ShadowScale = 0xC633,
    SharedData = 0x8781,
    Sharpness1 = 0xA40A,
    Sharpness2 = 0xFE56,
    ShutterSpeedValue = 0x9201,
    Site = 0x84E0,
    SMaxSampleValue = 0x0155,
    SMinSampleValue = 0x0154,
    Smoothness = 0xFE57,
    Software = 0x0131,
    SonyCropSize = 0x74C8,
    SonyCropTopLeft = 0x74C7,
    SonyRawFileType = 0x7000,
    SonyToneCurve = 0x7010,
    SpatialFrequencyResponse1 = 0x920C,
    SpatialFrequencyResponse2 = 0xA20C,
    SpectralSensitivity = 0x8824,
    SR2Private = 0xC634,
    SRawType = 0xC6C5,
    StandardOutputSensitivity = 0x8831,
    StitchInfo = 0x4748,
    StoNits = 0x923F,
    StripByteCounts = 0x0117,
    StripOffsets = 0x0111,
    StripRowCounts = 0x022F,
    SubfileType = 0x00FE,
    SubIFD = 0x014A,
    SubjectArea = 0x9214,
    SubjectDistance = 0x9206,
    SubjectDistanceRange = 0xA40C,
    SubjectLocation = 0xA214,
    SubSecTime = 0x9290,
    SubSecTimeDigitized = 0x9292,
    SubSecTimeOriginal = 0x9291,
    SubTileBlockSize = 0xC71E,
    T4Options = 0x0124,
    T6Options = 0x0125,
    T82Options = 0x01B3,
    T88Options = 0x8782,
    TargetPrinter = 0x0151,
    TextureFormat = 0x8216,
    Thresholding = 0x0107,
    ThumbnailLength = 0x0202,
    ThumbnailOffset = 0x0201,
    TiffEpStandardID1 = 0x9216,
    TiffEpStandardID2 = 0xA216,
    TiffFxextensions = 0x877F,
    TileByteCounts = 0x0145,
    TileDepth = 0x80E6,
    TileLength = 0x0143,
    TileOffsets = 0x0144,
    TileWidth = 0x0142,
    TimeCodes = 0xC763,
    TimeZoneOffset = 0x882A,
    TransferFunction = 0x012D,
    TransferRange = 0x0156,
    Transformation = 0xBC02,
    TransparencyIndicator = 0x84EC,
    TrapIndicator = 0x84EF,
    TStop = 0xC772,
    UIC1Tag = 0x835C,
    UIC2Tag = 0x835D,
    UIC3Tag = 0x835E,
    UIC4Tag = 0x835F,
    Uncompressed = 0xBC03,
    UniqueCameraModel = 0xC614,
    UserComment = 0x9286,
    USPTOMiscellaneous = 0x03E7,
    USPTOOriginalContentType = 0xC580,
    VersionYear = 0x0194,
    VignettingCorrection = 0x7031,
    VignettingCorrParams = 0x7032,
    WangAnnotation = 0x80A4,
    WangTag1 = 0x80A3,
    WangTag3 = 0x80A5,
    WangTag4 = 0x80A6,
    WarpQuadrilateral = 0x80BB,
    WaterDepth = 0x9403,
    WbGrgbLevels = 0x8602,
    WhiteBalance1 = 0xA403,
    WhiteBalance2 = 0xFE4E,
    WhiteLevel = 0xC61D,
    WhitePoint = 0x013E,
    WidthResolution = 0xBC82,
    WrapModes = 0x8217,
    XClipPathUnits = 0x0158,
    XpDipXml = 0x4747,
    XPAuthor = 0x9C9D,
    XPComment = 0x9C9C,
    XPKeywords = 0x9C9E,
    XPosition = 0x011E,
    XPSubject = 0x9C9F,
    XPTitle = 0x9C9B,
    XResolution = 0x011A,
    YCbCrCoefficients = 0x0211,
    YCbCrPositioning = 0x0213,
    YCbCrSubSampling = 0x0212,
    YClipPathUnits = 0x0159,
    YPosition = 0x011F,
    YResolution = 0x011B,
    Unknown = 0xFFFF // Not a real tag, shouldn't exist, value discrepant already exists as PlanarConfiguration 0x011c/284
}

impl From<u16> for ExifTag {
    fn from(val: u16) -> Self {
        match val as isize {
            0x9404 => Self::Acceleration,
            0xc68d => Self::ActiveArea,
            0x8336 => Self::AdventRevision,
            0x8335 => Self::AdventScale,
            0x8568 => Self::AfcpIptc,
            0x80bc => Self::AffineTransformMat,
            0xc660 => Self::AliasLayerMetadata,
            0xbcc3 => Self::AlphaByteCount,
            0xbcc5 => Self::AlphaDataDiscard,
            0xbcc2 => Self::AlphaOffset,
            0x9400 => Self::AmbientTemperature,
            0xc627 => Self::AnalogBalance,
            0xc44f => Self::Annotations,
            0xc632 => Self::AntiAliasStrength,
            0x9202 => Self::ApertureValue,
            0x02bc => Self::ApplicationNotes,
            0x013b => Self::Artist,
            0xc68f => Self::AsShotICCProfile,
            0xc628 => Self::AsShotNeutral,
            0xc690 => Self::AsShotPreProfileMatrix,
            0xc6f6 => Self::AsShotProfileName,
            0xc629 => Self::AsShotWhiteXY,
            0x84e8 => Self::BackgroundColorIndicator,
            0x84ea => Self::BackgroundColorValue,
            0x0146 => Self::BadFaxLines,
            0xc62a => Self::BaselineExposure,
            0xc7a5 => Self::BaselineExposureOffset,
            0xc62b => Self::BaselineNoise,
            0xc62c => Self::BaselineSharpness,
            0x828f => Self::BatteryLevel,
            0xc62d => Self::BayerGreenSplit,
            0xc65c => Self::BestQualityScale,
            0x84e5 => Self::BitsPerExtendedRunLength,
            0x84e4 => Self::BitsPerRunLength,
            0x0102 => Self::BitsPerSample,
            0xc61a => Self::BlackLevel,
            0xc61b => Self::BlackLevelDeltaH,
            0xc61c => Self::BlackLevelDeltaV,
            0xc619 => Self::BlackLevelRepeatDim,
            0xfe53 => Self::Brightness,
            0x9203 => Self::BrightnessValue,
            0xc7aa => Self::CacheVersion,
            0xc65a => Self::CalibrationIlluminant1,
            0xc65b => Self::CalibrationIlluminant2,
            0xc623 => Self::CameraCalibration1,
            0xc624 => Self::CameraCalibration2,
            0xc6f3 => Self::CameraCalibrationSig,
            0x9405 => Self::CameraElevationAngle,
            0xc7a1 => Self::CameraLabel,
            0xc62f => Self::CameraSerialNumber,
            0x0109 => Self::CellLength,
            0x0108 => Self::CellWidth,
            0xc617 => Self::CFALayout,
            0xa302 => Self::CFAPattern,
            0x828e => Self::CFAPattern2,
            0xc616 => Self::CFAPlaneColor,
            0x828d => Self::CFARepeatPatternDim,
            0xc631 => Self::ChromaBlurRadius,
            0x7034 => Self::ChromaticAberrationCorrection,
            0x7035 => Self::ChromaticAberrationCorrParams,
            0x923a => Self::CIP3DataFile,
            0x923b => Self::CIP3Sheet,
            0x923c => Self::CIP3Side,
            0x0147 => Self::CleanFaxData,
            0x0157 => Self::ClipPath,
            0x84f0 => Self::CMYKEquivalent,
            0x0193 => Self::CodingMethods,
            0x84ed => Self::ColorCharacterization,
            0xc6bf => Self::ColorimetricReference,
            0x0140 => Self::ColorMap,
            0xc621 => Self::ColorMatrix1,
            0xc622 => Self::ColorMatrix2,
            0x012c => Self::ColorResponseUnit,
            0x84e1 => Self::ColorSequence,
            0xa001 => Self::ColorSpace,
            0x84e6 => Self::ColorTable,
            0x9101 => Self::ComponentsConfiguration,
            0xa460 => Self::CompositeImage,
            0xa461 => Self::CompositeImageCount,
            0xa462 => Self::CompositeImageExposureTimes,
            0x9102 => Self::CompressedBitsPerPixel,
            0x0103 => Self::Compression,
            0x0148 => Self::ConsecutiveBadFaxLines,
            0xa408 => Self::Contrast1,
            0xfe54 => Self::Contrast2,
            0xfe4d => Self::Converter,
            0x8298 => Self::Copyright,
            0xc5e0 => Self::CR2CFAPattern,
            0x9004 => Self::CreateDate,
            0xc691 => Self::CurrentICCProfile,
            0xc692 => Self::CurrentPreProfileMatrix,
            0xa401 => Self::CustomRendered,
            0x80e4 => Self::DataType,
            0x9003 => Self::DateTimeOriginal,
            0x01b1 => Self::Decode,
            0xc7a6 => Self::DefaultBlackRender,
            0xc61f => Self::DefaultCropOrigin,
            0xc620 => Self::DefaultCropSize,
            0x01b2 => Self::DefaultImageColor,
            0xc61e => Self::DefaultScale,
            0xc7b5 => Self::DefaultUserCrop,
            0xc7eb => Self::DepthFar,
            0xc7e9 => Self::DepthFormat,
            0xc7ed => Self::DepthMeasureType,
            0xc7ea => Self::DepthNear,
            0xc7ec => Self::DepthUnits,
            0xa40b => Self::DeviceSettingDescription,
            0xa404 => Self::DigitalZoomRatio,
            0x7036 => Self::DistortionCorrection,
            0x7037 => Self::DistortionCorrParams,
            0xc613 => Self::DNGBackwardVersion,
            0xc630 => Self::DNGLensInfo,
            0xc612 => Self::DNGVersion,
            0x010d => Self::DocumentName,
            0x0150 => Self::DotRange,
            0xc7ee => Self::EnhanceParams,
            0xa003 => Self::ExifImageHeight,
            0xa002 => Self::ExifImageWidth,
            0x8769 => Self::ExifOffset,
            0x9000 => Self::ExifVersion,
            0xafc2 => Self::ExpandFilm,
            0xafc3 => Self::ExpandFilterLens,
            0xafc5 => Self::ExpandFlashLamp,
            0xafc1 => Self::ExpandLens,
            0xafc4 => Self::ExpandScanner,
            0xafc0 => Self::ExpandSoftware,
            0xfe51 => Self::Exposure,
            0x9204 => Self::ExposureCompensation,
            0x9215 => Self::ExposureIndex1,
            0xa215 => Self::ExposureIndex2,
            0xa402 => Self::ExposureMode,
            0x8822 => Self::ExposureProgram,
            0x829a => Self::ExposureTime,
            0x0152 => Self::ExtraSamples,
            0x0192 => Self::FaxProfile,
            0x885c => Self::FaxRecvParams,
            0x885e => Self::FaxRecvTime,
            0x885d => Self::FaxSubAddress,
            0x8871 => Self::FedexEDR,
            0xa300 => Self::FileSource,
            0x010a => Self::FillOrder,
            0x9209 => Self::Flash,
            0x920b => Self::FlashEnergy1,
            0xa20b => Self::FlashEnergy2,
            0xa000 => Self::FlashpixVersion,
            0x829d => Self::FNumber,
            0x920a => Self::FocalLength,
            0xa405 => Self::FocalLengthIn35mmFormat,
            0x9210 => Self::FocalPlaneResolutionUnit1,
            0xa210 => Self::FocalPlaneResolutionUnit2,
            0x920e => Self::FocalPlaneXResolution1,
            0xa20e => Self::FocalPlaneXResolution2,
            0x920f => Self::FocalPlaneYResolution1,
            0xa20f => Self::FocalPlaneYResolution2,
            0xc714 => Self::ForwardMatrix1,
            0xc715 => Self::ForwardMatrix2,
            0x8218 => Self::FovCot,
            0xc764 => Self::FrameRate,
            0x0121 => Self::FreeByteCounts,
            0x0120 => Self::FreeOffsets,
            0xa407 => Self::GainControl,
            0xa500 => Self::Gamma,
            0xa480 => Self::GDALMetadata,
            0xa481 => Self::GDALNoData,
            0x87b1 => Self::GeoTiffAsciiParams,
            0x87af => Self::GeoTiffDirectory,
            0x87b0 => Self::GeoTiffDoubleParams,
            0x0190 => Self::GlobalParametersIFD,
            0x9009 => Self::GooglePlusUploadCode,
            0x8825 => Self::GPSInfo,
            0x0123 => Self::GrayResponseCurve,
            0x0122 => Self::GrayResponseUnit,
            0x0141 => Self::HalftoneHints,
            0xc51b => Self::HasselbladExif,
            0xb4c3 => Self::HasselbladRawImage,
            0x84ee => Self::HCUsage,
            0xbc83 => Self::HeightResolution,
            0x013c => Self::HostComputer,
            0x9401 => Self::Humidity,
            0x8773 => Self::IccProfile,
            0xbcc1 => Self::ImageByteCount,
            0x84e7 => Self::ImageColorIndicator,
            0x84e9 => Self::ImageColorValue,
            0xbcc4 => Self::ImageDataDiscard,
            0x80e5 => Self::ImageDepth,
            0x010e => Self::ImageDescription,
            0x8215 => Self::ImageFullHeight,
            0x8214 => Self::ImageFullWidth,
            0x0101 => Self::ImageHeight1,
            0xbc81 => Self::ImageHeight2,
            0x9213 => Self::ImageHistory1,
            0xa213 => Self::ImageHistory2,
            0x800d => Self::ImageID,
            0x87ac => Self::ImageLayer,
            0x9211 => Self::ImageNumber1,
            0xa211 => Self::ImageNumber2,
            0xbcc0 => Self::ImageOffset,
            0x80b9 => Self::ImageReferencePoints,
            0x935c => Self::ImageSourceData,
            0xbc04 => Self::ImageType,
            0xa420 => Self::ImageUniqueID,
            0x0100 => Self::ImageWidth1,
            0xbc80 => Self::ImageWidth2,
            0x015a => Self::Indexed,
            0x8481 => Self::INGRReserved,
            0x014d => Self::InkNames,
            0x014c => Self::InkSet,
            0x847f => Self::IntergraphFlagRegisters,
            0x8480 => Self::IntergraphMatrix,
            0x847e => Self::IntergraphPacketData,
            0x8829 => Self::Interlace,
            0x0001 => Self::InteropIndex,
            0xa005 => Self::InteropOffset,
            0x0002 => Self::InteropVersion,
            0x83bb => Self::IPTC,
            0x8827 => Self::ISO,
            0x8833 => Self::ISOSpeed,
            0x8834 => Self::ISOSpeedLatitudeyyy,
            0x8835 => Self::ISOSpeedLatitudezzz,
            0x84e2 => Self::IT8Header,
            0x87be => Self::JBIGOptions,
            0x0209 => Self::JPEGACTables,
            0x0208 => Self::JPEGDCTables,
            0x0205 => Self::JPEGLosslessPredictors,
            0x0206 => Self::JPEGPointTransforms,
            0x0200 => Self::JPEGProc,
            0x0207 => Self::JPEGQTables,
            0x0203 => Self::JPEGRestartInterval,
            0x015b => Self::JPEGTables1,
            0x01b5 => Self::JPEGTables2,
            0x85d7 => Self::JPLCartoIFD,
            0xfe00 => Self::KdcIfd,
            0x8290 => Self::KodakIFD,
            0x8606 => Self::LeafData,
            0x888a => Self::LeafSubIFD,
            0xfdea => Self::Lens,
            0xa432 => Self::LensInfo,
            0xa433 => Self::LensMake,
            0xa434 => Self::LensModel,
            0xa435 => Self::LensSerialNumber,
            0x9208 => Self::LightSource,
            0xc618 => Self::LinearizationTable,
            0xc62e => Self::LinearResponseLimit,
            0xc615 => Self::LocalizedCameraModel,
            0x010f => Self::Make,
            0x927c => Self::MakerNoteApple,
            0xc635 => Self::MakerNoteSafety,
            0xc68e => Self::MaskedAreas,
            0x821a => Self::MatrixWorldToCamera,
            0x8219 => Self::MatrixWorldToScreen,
            0x80e3 => Self::Matteing,
            0x9205 => Self::MaxApertureValue,
            0x0119 => Self::MaxSampleValue,
            0x82a7 => Self::MDColorTable,
            0x82a5 => Self::MDFileTag,
            0x82ac => Self::MDFileUnits,
            0x82a8 => Self::MDLabName,
            0x82aa => Self::MDPrepDate,
            0x82ab => Self::MDPrepTime,
            0x82a9 => Self::MDSampleInfo,
            0x82a6 => Self::MDScalePixel,
            0x9207 => Self::MeteringMode,
            0x0118 => Self::MinSampleValue,
            0x0110 => Self::Model,
            0x827d => Self::Model2,
            0x8482 => Self::ModelTiePoint,
            0x85d8 => Self::ModelTransform,
            0x0195 => Self::ModeNumber,
            0x0132 => Self::ModifyDate,
            0xfe58 => Self::MoireFilter,
            0x932f => Self::MSDocumentText,
            0x9331 => Self::MSDocumentTextPosition,
            0x9330 => Self::MSPropertySetStorage,
            0x8780 => Self::MultiProfiles,
            0xc7a7 => Self::NewRawImageDigest,
            0xc7d5 => Self::NikonNEFInfo,
            0x920d => Self::Noise1,
            0xa20d => Self::Noise2,
            0xc761 => Self::NoiseProfile,
            0xc6f7 => Self::NoiseReductionApplied,
            0x014e => Self::NumberofInks,
            0xc428 => Self::OceApplicationSelector,
            0xc429 => Self::OceIDNumber,
            0xc42a => Self::OceImageLogic,
            0xc427 => Self::OceScanjobDesc,
            0xea1d => Self::OffsetSchema,
            0x9010 => Self::OffsetTime,
            0x9012 => Self::OffsetTimeDigitized,
            0x9011 => Self::OffsetTimeOriginal,
            0x00ff => Self::OldSubfileType,
            0xc740 => Self::OpcodeList1,
            0xc741 => Self::OpcodeList2,
            0xc74e => Self::OpcodeList3,
            0x015f => Self::OPIProxy,
            0x8828 => Self::Opto,
            0x0112 => Self::Orientation,
            0xc792 => Self::OriginalBestQualitySize,
            0xc793 => Self::OriginalDefaultCropSize,
            0xc791 => Self::OriginalDefaultFinalSize,
            0xc573 => Self::OriginalFileName,
            0xc68c => Self::OriginalRawFileData,
            0xc71d => Self::OriginalRawFileDigest,
            0xc68b => Self::OriginalRawFileName,
            0xa430 => Self::OwnerName1,
            0xfde8 => Self::OwnerName2,
            0xea1c => Self::Padding,
            0x011d => Self::PageName,
            0x0129 => Self::PageNumber,
            0xc6d2 => Self::PanasonicTitle,
            0xc6d3 => Self::PanasonicTitle2,
            0x0106 => Self::PhotometricInterpretation,
            0x8649 => Self::PhotoshopSettings,
            0xbc01 => Self::PixelFormat,
            0x84eb => Self::PixelIntensityRange,
            0x85b8 => Self::PixelMagicJBIGOptions,
            0x830e => Self::PixelScale,
            0x011c => Self::PlanarConfiguration,
            0x013d => Self::Predictor,
            0x9402 => Self::Pressure,
            0xc716 => Self::PreviewApplicationName,
            0xc717 => Self::PreviewApplicationVersion,
            0xc71a => Self::PreviewColorSpace,
            0xc71b => Self::PreviewDateTime,
            0xc719 => Self::PreviewSettingsDigest,
            0xc718 => Self::PreviewSettingsName,
            0x013f => Self::PrimaryChromaticities,
            0xc4a5 => Self::PrintIM,
            0x000b => Self::ProcessingSoftware,
            0xc6f4 => Self::ProfileCalibrationSig,
            0xc6fe => Self::ProfileCopyright,
            0xc6fd => Self::ProfileEmbedPolicy,
            0xc6fa => Self::ProfileHueSatMapData1,
            0xc6fb => Self::ProfileHueSatMapData2,
            0xc6f9 => Self::ProfileHueSatMapDims,
            0xc7a3 => Self::ProfileHueSatMapEncoding,
            0xc6f5 => Self::ProfileIFD,
            0xc726 => Self::ProfileLookTableData,
            0xc725 => Self::ProfileLookTableDims,
            0xc7a4 => Self::ProfileLookTableEncoding,
            0xc6f8 => Self::ProfileName,
            0xc6fc => Self::ProfileToneCurve,
            0x0191 => Self::ProfileType,
            0x84e3 => Self::RasterPadding,
            0x4746 => Self::Rating,
            0x4749 => Self::RatingPercent,
            0xc65d => Self::RawDataUniqueID,
            0xfe4c => Self::RawFile,
            0xc71c => Self::RawImageDigest,
            0xc640 => Self::RawImageSegmentation,
            0xc7a8 => Self::RawToPreviewGain,
            0x8832 => Self::RecommendedExposureIndex,
            0xc625 => Self::ReductionMatrix1,
            0xc626 => Self::ReductionMatrix2,
            0xc789 => Self::ReelName,
            0x0214 => Self::ReferenceBlackWhite,
            0x80ba => Self::RegionXformTackPoint,
            0x1000 => Self::RelatedImageFileFormat,
            0x1002 => Self::RelatedImageHeight,
            0x1001 => Self::RelatedImageWidth,
            0xa004 => Self::RelatedSoundFile,
            0x0128 => Self::ResolutionUnit,
            0xc71f => Self::RowInterleaveFactor,
            0x0116 => Self::RowsPerStrip,
            0x0153 => Self::SampleFormat,
            0x0115 => Self::SamplesPerPixel,
            0xa101 => Self::SamsungRawByteOrder,
            0xa011 => Self::SamsungRawPointersLength,
            0xa010 => Self::SamsungRawPointersOffset,
            0xa102 => Self::SamsungRawUnknown,
            0xa409 => Self::Saturation1,
            0xfe55 => Self::Saturation2,
            0xa406 => Self::SceneCaptureType,
            0xa301 => Self::SceneType,
            0x9212 => Self::SecurityClassification1,
            0xa212 => Self::SecurityClassification2,
            0x882b => Self::SelfTimerMode,
            0x8546 => Self::SEMInfo,
            0x9217 => Self::SensingMethod1,
            0xa217 => Self::SensingMethod2,
            0x8830 => Self::SensitivityType,
            0xa431 => Self::SerialNumber1,
            0xfde9 => Self::SerialNumber2,
            0xfe52 => Self::Shadows,
            0xc633 => Self::ShadowScale,
            0x8781 => Self::SharedData,
            0xa40a => Self::Sharpness1,
            0xfe56 => Self::Sharpness2,
            0x9201 => Self::ShutterSpeedValue,
            0x84e0 => Self::Site,
            0x0155 => Self::SMaxSampleValue,
            0x0154 => Self::SMinSampleValue,
            0xfe57 => Self::Smoothness,
            0x0131 => Self::Software,
            0x74c8 => Self::SonyCropSize,
            0x74c7 => Self::SonyCropTopLeft,
            0x7000 => Self::SonyRawFileType,
            0x7010 => Self::SonyToneCurve,
            0x920c => Self::SpatialFrequencyResponse1,
            0xa20c => Self::SpatialFrequencyResponse2,
            0x8824 => Self::SpectralSensitivity,
            0xc634 => Self::SR2Private,
            0xc6c5 => Self::SRawType,
            0x8831 => Self::StandardOutputSensitivity,
            0x4748 => Self::StitchInfo,
            0x923f => Self::StoNits,
            0x0117 => Self::StripByteCounts,
            0x0111 => Self::StripOffsets,
            0x022f => Self::StripRowCounts,
            0x00fe => Self::SubfileType,
            0x014a => Self::SubIFD,
            0x9214 => Self::SubjectArea,
            0x9206 => Self::SubjectDistance,
            0xa40c => Self::SubjectDistanceRange,
            0xa214 => Self::SubjectLocation,
            0x9290 => Self::SubSecTime,
            0x9292 => Self::SubSecTimeDigitized,
            0x9291 => Self::SubSecTimeOriginal,
            0xc71e => Self::SubTileBlockSize,
            0x0124 => Self::T4Options,
            0x0125 => Self::T6Options,
            0x01b3 => Self::T82Options,
            0x8782 => Self::T88Options,
            0x0151 => Self::TargetPrinter,
            0x8216 => Self::TextureFormat,
            0x0107 => Self::Thresholding,
            0x0202 => Self::ThumbnailLength,
            0x0201 => Self::ThumbnailOffset,
            0x9216 => Self::TiffEpStandardID1,
            0xa216 => Self::TiffEpStandardID2,
            0x877f => Self::TiffFxextensions,
            0x0145 => Self::TileByteCounts,
            0x80e6 => Self::TileDepth,
            0x0143 => Self::TileLength,
            0x0144 => Self::TileOffsets,
            0x0142 => Self::TileWidth,
            0xc763 => Self::TimeCodes,
            0x882a => Self::TimeZoneOffset,
            0x012d => Self::TransferFunction,
            0x0156 => Self::TransferRange,
            0xbc02 => Self::Transformation,
            0x84ec => Self::TransparencyIndicator,
            0x84ef => Self::TrapIndicator,
            0xc772 => Self::TStop,
            0x835c => Self::UIC1Tag,
            0x835d => Self::UIC2Tag,
            0x835e => Self::UIC3Tag,
            0x835f => Self::UIC4Tag,
            0xbc03 => Self::Uncompressed,
            0xc614 => Self::UniqueCameraModel,
            0x9286 => Self::UserComment,
            0x03e7 => Self::USPTOMiscellaneous,
            0xc580 => Self::USPTOOriginalContentType,
            0x0194 => Self::VersionYear,
            0x7031 => Self::VignettingCorrection,
            0x7032 => Self::VignettingCorrParams,
            0x80a4 => Self::WangAnnotation,
            0x80a3 => Self::WangTag1,
            0x80a5 => Self::WangTag3,
            0x80a6 => Self::WangTag4,
            0x80bb => Self::WarpQuadrilateral,
            0x9403 => Self::WaterDepth,
            0x8602 => Self::WbGrgbLevels,
            0xa403 => Self::WhiteBalance1,
            0xfe4e => Self::WhiteBalance2,
            0xc61d => Self::WhiteLevel,
            0x013e => Self::WhitePoint,
            0xbc82 => Self::WidthResolution,
            0x8217 => Self::WrapModes,
            0x0158 => Self::XClipPathUnits,
            0x4747 => Self::XpDipXml,
            0x9c9d => Self::XPAuthor,
            0x9c9c => Self::XPComment,
            0x9c9e => Self::XPKeywords,
            0x011e => Self::XPosition,
            0x9c9f => Self::XPSubject,
            0x9c9b => Self::XPTitle,
            0x011a => Self::XResolution,
            0x0211 => Self::YCbCrCoefficients,
            0x0213 => Self::YCbCrPositioning,
            0x0212 => Self::YCbCrSubSampling,
            0x0159 => Self::YClipPathUnits,
            0x011f => Self::YPosition,
            0x011b => Self::YResolution,
            _ => Self::Unknown // set to 0xFFFF in enum but shouldn't exist
        }
    }
}