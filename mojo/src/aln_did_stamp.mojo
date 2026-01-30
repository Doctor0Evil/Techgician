struct AlnDidBostromStampV1:
    var author_system: String
    var primary_bostrom_addr: String
    var alt_bostrom_addr: String
    var has_alt_bostrom_addr: Bool
    var safe_addrs: List[String]
    var response_hash_hex: String
    var T_score_0_to_1: Float64
    var P_score_0_to_1: Float64
    var R_score_0_to_1: Float64
    var C_score_0_to_1: Float64
    var timestamp_utc_iso8601: String
    var notes: String
    var has_notes: Bool

    fn init(
        author_system: String,
        primary_bostrom_addr: String,
        safe_addrs: List[String],
        response_hash_hex: String,
        T_score_0_to_1: Float64,
        P_score_0_to_1: Float64,
        R_score_0_to_1: Float64,
        C_score_0_to_1: Float64,
        timestamp_utc_iso8601: String,
        alt_bostrom_addr: String = "",
        has_alt_bostrom_addr: Bool = False,
        notes: String = "",
        has_notes: Bool = False
    ):
        self.author_system = author_system
        self.primary_bostrom_addr = primary_bostrom_addr
        self.safe_addrs = safe_addrs
        self.response_hash_hex = response_hash_hex
        self.T_score_0_to_1 = T_score_0_to_1
        self.P_score_0_to_1 = P_score_0_to_1
        self.R_score_0_to_1 = R_score_0_to_1
        self.C_score_0_to_1 = C_score_0_to_1
        self.timestamp_utc_iso8601 = timestamp_utc_iso8601
        self.alt_bostrom_addr = alt_bostrom_addr
        self.has_alt_bostrom_addr = has_alt_bostrom_addr
        self.notes = notes
        self.has_notes = has_notes
