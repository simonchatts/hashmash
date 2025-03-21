//! Construct a 4kb table containing the most frequent English trigrams as a
//! bitmap. So consider each letter as a number from 0 ..= 25, using 5 bits, and
//! thus a trigram as a 15-bit index into a bitmap. If the bit is set, then the
//! trigram is in the top 15% of English trigrams.
//!
//! This is a utility executable categorised to Cargo as an example, to avoid
//! polluting the regular package. So to regenerate `src/trigrams.bitmap`, run
//! `cargo run --example build_tri`.

use std::fs::File;
use std::io::{self, Write};

/// Main: just construct the bitmap and write it out as a 4kb file. The
/// `classify.rs` module embeds this into its executable's read-only data
/// segment when it is compiled.
fn main() {
    let mut bitmap = Bitmap::new();
    for tri in TOP_TRIGRAMS.iter() {
        let chars: Vec<usize> = tri.chars().map(|c| c as usize - 'A' as usize).collect();
        bitmap.set_tri(chars[0], chars[1], chars[2]);
    }
    let filename = "src/trigrams.bitmap";
    bitmap
        .write(&filename)
        .unwrap_or_else(|err| eprintln!("Can't write to {}: {}", &filename, err))
}

/// 4kb bitmap (mapping a 15-bit key to 1-bit of data)
struct Bitmap {
    bytes: [u8; 4096],
}

impl Bitmap {
    fn new() -> Self {
        Bitmap { bytes: [0; 4096] }
    }

    fn set(&mut self, bit: usize) {
        let byte = bit / 8;
        let bit_index = bit % 8;
        let mask = 1 << bit_index;
        self.bytes[byte] |= mask;
    }

    fn set_tri(&mut self, word0: usize, word1: usize, word2: usize) {
        self.set(word0 | word1 << 5 | word2 << 10)
    }

    fn write(&self, filename: &str) -> io::Result<()> {
        File::create(filename)?.write_all(&self.bytes)
    }
}

/// The top 15% of English letter triagrams (in decreasing order of frequency,
/// but we don't care about that). From
/// http://practicalcryptography.com/cryptanalysis/letter-frequencies-various-languages/english-letter-frequencies/
const TOP_TRIGRAMS: [&str; 2648] = [
    "THE", "AND", "ING", "ENT", "ION", "HER", "FOR", "THA", "NTH", "INT", "ERE", "TIO",
    "TER", "EST", "ERS", "ATI", "HAT", "ATE", "ALL", "ETH", "HES", "VER", "HIS", "OFT",
    "ITH", "FTH", "STH", "OTH", "RES", "ONT", "DTH", "ARE", "REA", "EAR", "WAS", "SIN",
    "STO", "TTH", "STA", "THI", "TIN", "TED", "ONS", "EDT", "WIT", "SAN", "DIN", "ORT",
    "CON", "RTH", "EVE", "ECO", "ERA", "IST", "NGT", "AST", "ILL", "COM", "ORE", "IVE",
    "NCE", "ONE", "EDI", "PRO", "ESS", "OUT", "EIN", "ATT", "MEN", "HEC", "ESA", "HEN",
    "INA", "ERI", "ERT", "AME", "ITI", "OME", "SON", "ART", "MAN", "EAN", "ONA", "EOF",
    "TOR", "HEA", "RAN", "RIN", "INE", "EDA", "NTO", "AVE", "NIN", "OVE", "OUN", "AIN",
    "ANT", "STR", "ETO", "HEM", "SOF", "PER", "NDE", "STE", "NTE", "EAS", "DTO", "OUR",
    "RED", "ROM", "TOF", "GHT", "TOT", "ESE", "CHA", "ICA", "HEI", "HIN", "IDE", "NDT",
    "HAN", "TAN", "LIN", "NOT", "DER", "ECT", "TRA", "IGH", "FRO", "EAT", "STI", "HEP",
    "NDI", "INS", "SHE", "NAL", "PLA", "ALS", "EEN", "NTI", "YOU", "LAN", "UND", "NDA",
    "RAT", "LEA", "CAN", "HAS", "NDS", "NGA", "HEL", "HED", "INC", "USE", "ESI", "GTH",
    "ASA", "HET", "NTS", "HAV", "HEW", "THO", "BUT", "NAN", "ASS", "HEF", "IES", "RET",
    "END", "PAR", "WER", "CTI", "REN", "REC", "CAL", "ITS", "REE", "ENE", "RST", "EAL",
    "ANA", "NST", "COU", "TUR", "MIN", "ITY", "YTH", "HEY", "ECA", "OUL", "LLE", "ARD",
    "ROU", "ANC", "OST", "PRE", "AGE", "EFO", "LES", "SSI", "EMA", "ESO", "TAT", "ATH",
    "WOR", "UST", "HEB", "EWA", "SHO", "IND", "SED", "HOU", "LLY", "ULD", "ASE", "URE",
    "ONO", "ELE", "ENC", "NAT", "EAD", "WHE", "ELL", "BLE", "KIN", "ANS", "TIC", "ALI",
    "SCO", "ERO", "WHI", "CES", "OWN", "NTA", "ACT", "BER", "VEN", "TIM", "DON", "DAN",
    "OSE", "ICE", "ISA", "TON", "DEN", "NGS", "UGH", "NES", "LAT", "TAL", "EDO", "TEN",
    "IME", "EME", "ACK", "TES", "PLE", "OUS", "OFF", "TTO", "CHI", "ANI", "ORM", "NED",
    "ENS", "SHA", "MOR", "ISS", "ITE", "NGE", "TIS", "ORA", "LLI", "EDE", "SSE", "ADE",
    "RIE", "AID", "EMO", "RAL", "SIT", "OIN", "HTH", "TRE", "ANY", "AKE", "ERN", "MER",
    "RIC", "DIS", "ISH", "OUG", "INI", "ONG", "NTR", "ELI", "WIL", "LED", "SAR", "HOW",
    "EDB", "ICH", "SPE", "SEA", "LIT", "YIN", "SAI", "NDO", "GIN", "SHI", "ORD", "MON",
    "ENA", "NEW", "POR", "SER", "IAL", "ORI", "TTE", "MAR", "EPR", "ACH", "HAR", "YEA",
    "TRI", "CHE", "TEA", "UNT", "OMP", "WHO", "TAR", "OWE", "RIT", "DED", "ORS", "DAY",
    "HEE", "THR", "EIR", "OND", "MES", "EFI", "HAD", "NER", "ELA", "LET", "LSO", "RIS",
    "IRE", "ISI", "MET", "ARS", "HIC", "CEN", "ARI", "FIN", "TOB", "NSI", "LAS", "OPE",
    "LAR", "DES", "FTE", "NIT", "SEN", "ANG", "SOM", "ABO", "SIO", "TWO", "IAN", "EIS",
    "TSA", "NGI", "UNI", "SES", "REP", "RAC", "TOP", "ABL", "ETI", "EBE", "EHA", "NOW",
    "ONI", "VES", "FIR", "ERC", "OFA", "ACE", "SAL", "GET", "APP", "ANE", "RSA", "NOF",
    "HEH", "GRE", "WIN", "CAR", "ETE", "MAT", "CHO", "LAY", "SWE", "ESP", "PRI", "TIV",
    "ROF", "GRA", "LLO", "COR", "EAC", "NIS", "DIT", "GAN", "GTO", "ENO", "BOU", "OBE",
    "ESH", "TOS", "ERY", "RMA", "NGO", "EWI", "ARA", "RTO", "REL", "OMA", "ALA", "ASI",
    "TST", "UTT", "IRS", "YAN", "LLA", "SFO", "ORK", "ETT", "LTH", "SID", "ASO", "SWI",
    "ITA", "SET", "TWA", "ERM", "EPA", "RON", "TIT", "AFT", "DRE", "TLE", "MIL", "DBY",
    "ALE", "PEN", "BEC", "MBE", "TOA", "HEG", "SCH", "SIS", "RTI", "HEO", "LOW", "LIS",
    "OLL", "WAR", "ALT", "ELO", "TRO", "CAT", "MED", "LIC", "HIL", "ILE", "THT", "REM",
    "RRE", "AYS", "OLI", "RSO", "NSA", "OMM", "OLD", "CRE", "ATA", "ISE", "CIA", "POS",
    "GER", "SMA", "UTI", "STS", "SEC", "SBE", "ENI", "SRE", "LON", "ISC", "NSE", "NOR",
    "BEE", "ANO", "NCO", "FER", "ITT", "SNO", "EPO", "EON", "EDS", "EAM", "ESC", "FIC",
    "ECH", "WAY", "VED", "IKE", "ALO", "YOF", "ASH", "OTE", "OOK", "ETA", "ERF", "ONC",
    "EMI", "ECI", "ATS", "ERV", "RSI", "SST", "ILI", "EED", "ARY", "SSO", "MTH", "VEL",
    "DAT", "MEA", "ESU", "URI", "PAN", "RCH", "UTH", "SPO", "WOU", "FFE", "SEL", "REI",
    "RGE", "RSE", "TOM", "USI", "EGA", "SAS", "SSA", "ATO", "ERW", "OOD", "AMA", "SAT",
    "ECE", "MPL", "TSO", "GEN", "ARR", "DEA", "SCA", "DOF", "UAL", "DBE", "EWO", "NSO",
    "RTE", "VIN", "ADI", "NDW", "NDH", "EDF", "SWH", "SEE", "TOC", "TCH", "EWH", "EBA",
    "ONL", "TEM", "DWI", "ERR", "LEC", "LAC", "EOP", "TEL", "AMI", "EHE", "DFO", "IEN",
    "UCH", "NDC", "ELY", "DST", "ICI", "EDW", "AUS", "NFO", "NTT", "NNE", "EWE", "SUR",
    "EXP", "BET", "KET", "INF", "ETR", "YTO", "RDE", "RCE", "OMT", "EVI", "VET", "PEC",
    "RAI", "ARL", "YST", "SOU", "HIM", "REF", "LIK", "GES", "CTO", "URN", "FOU", "LLS",
    "RNE", "WHA", "TOW", "NDR", "DAS", "SSU", "SPR", "OPL", "RAD", "ESW", "ONF", "COL",
    "OMI", "DUC", "MOS", "ARK", "TAI", "ICK", "HOS", "ULT", "EMB", "IGN", "HOO", "TOO",
    "NEA", "ITW", "PPE", "FFI", "ULA", "NAM", "MIS", "CED", "LOS", "GAM", "NAS", "REG",
    "LIE", "OLO", "TWE", "ANN", "TOD", "AIL", "OTA", "ISO", "AYE", "TCO", "GRO", "CAM",
    "EFE", "ONW", "BEA", "NGL", "WEE", "EDU", "REW", "AGA", "TIL", "ODE", "ORY", "ERB",
    "BAC", "LEN", "NLY", "IMP", "ARO", "EHI", "AMP", "MPA", "SPA", "IVI", "ICT", "STT",
    "NET", "RTA", "HRE", "ERH", "EET", "ROW", "SOR", "EPE", "TIE", "NCH", "TRU", "MAL",
    "CTE", "NDM", "ATC", "FAC", "TWI", "ORC", "DCO", "NDP", "FRE", "OFS", "ARG", "IMA",
    "DEC", "ENG", "RIA", "OWI", "EIT", "POL", "KED", "INN", "BLI", "DOW", "ETW", "CLU",
    "HOL", "AIR", "RRI", "RIG", "SLA", "ROP", "OFI", "TOH", "NON", "OOL", "ISP", "MAK",
    "REV", "EQU", "LYT", "JUS", "LYA", "OLE", "CIT", "PEA", "VEA", "KNO", "OCA", "TAK",
    "ACC", "CER", "ADA", "SUP", "NGW", "PEO", "DSO", "RDI", "TOU", "CAU", "ROV", "TFO",
    "STU", "PIN", "TLY", "IED", "UES", "TSI", "AWA", "LER", "WEL", "EPL", "HOR", "ERP",
    "HRO", "UTE", "OSS", "TBE", "TYO", "TOG", "HEU", "BYT", "NGF", "NGH", "HAL", "BAN",
    "ISM", "ROL", "ATU", "SAM", "HOM", "DHI", "ILD", "GAI", "RDS", "EDH", "INO", "EGO",
    "EGI", "LLT", "ARC", "RAM", "OTO", "CLE", "DHE", "ACO", "ORL", "CAS", "RIV", "OFC",
    "ORN", "RNA", "URS", "AVI", "NAR", "UBL", "TTI", "MEM", "AMO", "RME", "QUI", "QUE",
    "NEE", "NDF", "INH", "INK", "EHO", "NCL", "OCK", "VIC", "RTS", "ETS", "LEG", "CLA",
    "TSE", "EOR", "NCI", "SLI", "ERG", "DID", "RCO", "BAS", "LOC", "EBU", "HIG", "ASP",
    "EXT", "EGR", "DIA", "TAB", "BRI", "CEI", "RAS", "DAL", "ALC", "RUN", "PPO", "OFH",
    "CKE", "ATW", "CEA", "OTT", "BAL", "NSH", "MAI", "DAR", "ASB", "RFO", "VIS", "OWA",
    "UDE", "RSH", "IFI", "LOO", "HIT", "UAR", "CET", "OVI", "NDB", "TOL", "UTO", "LEM",
    "BRO", "VID", "RIO", "UCT", "TRY", "EFR", "SDE", "DHA", "BEL", "BIL", "TAS", "SUC",
    "EBO", "ORO", "SEV", "MIT", "YCO", "NWH", "NEO", "RVI", "DEV", "EUN", "WAN", "SIG",
    "THS", "ASC", "DSA", "TAC", "EMP", "ANK", "RKE", "NHE", "NME", "NRE", "ROS", "NGC",
    "MAY", "UNC", "NIC", "NNI", "URA", "FUL", "OCO", "OSI", "DEF", "NGR", "TEC", "SAY",
    "NBE", "YRE", "NTL", "DGE", "SAC", "SFR", "TSH", "ISN", "LDI", "ARM", "CUR", "EAG",
    "NDL", "ELD", "SMO", "ORG", "LLB", "DEL", "POI", "IAT", "URT", "KER", "SOL", "TMA",
    "MUS", "BEI", "QUA", "LYS", "DWA", "NMA", "HIP", "ROD", "EFA", "INV", "ISL", "PON",
    "YON", "PAS", "LIA", "REO", "LIF", "TIA", "CRI", "NSU", "RLY", "LBE", "OHA", "NDD",
    "HAM", "EPT", "EBR", "BES", "NGP", "YHA", "TME", "LEF", "ADD", "OFM", "MAD", "DMA",
    "DUR", "MME", "MAS", "IZE", "ICS", "YBE", "DNO", "EGE", "GAT", "DDE", "CUL", "UTA",
    "MEO", "LCO", "VAL", "ATR", "ECU", "GED", "NDU", "YAR", "DET", "ISF", "OFP", "SPI",
    "ROT", "ROB", "PED", "LDE", "OTI", "EEK", "DEM", "BEF", "MOV", "ELS", "IOU", "SWA",
    "ERL", "IMI", "FAM", "NEX", "LOF", "FAN", "OES", "SCR", "ISW", "ULL", "ERD", "INM",
    "SOC", "RNI", "EEL", "APA", "MIC", "DEP", "RDA", "PIT", "OGR", "SUN", "ESF", "IDA",
    "GAR", "CKS", "RAG", "GEO", "GOO", "LUD", "NHI", "IFE", "BRA", "DDI", "PAT", "CEO",
    "NGB", "KES", "YER", "NWA", "DRA", "LYI", "ODU", "PUB", "INU", "ORR", "AYA", "LOR",
    "TTL", "HON", "SAB", "NHA", "LDB", "LIV", "EDR", "SEO", "OAD", "RMO", "DLE", "ECR",
    "UDI", "GON", "ALF", "DIE", "RWA", "LST", "LIG", "RID", "RMI", "OFE", "URC", "SIC",
    "APE", "AYI", "BOT", "EYE", "FRI", "ILA", "RYO", "ADO", "ALM", "SAG", "NWI", "WED",
    "NNO", "ABI", "RVE", "EES", "AKI", "FIE", "BOR", "DOU", "EEP", "DIF", "TET", "ORP",
    "ITO", "AYT", "ONH", "OBA", "TUD", "VIE", "NYO", "SBU", "DIC", "DRI", "OFO", "OAN",
    "RYA", "RUS", "XPE", "NCA", "NVE", "ILY", "HOT", "NAG", "UTS", "LRE", "RAP", "ECL",
    "DSE", "OUP", "LEO", "UPP", "ROA", "NAD", "BUS", "NTW", "ALP", "URR", "YFO", "OMO",
    "ONM", "NTU", "OWS", "IFF", "RSW", "ASW", "ASN", "HAP", "CCE", "CRO", "IER", "CCO",
    "CAP", "LEV", "TSW", "KEN", "OKE", "OWT", "ASM", "NGM", "CTS", "NOU", "PUT", "REB",
    "TTA", "EDM", "LIO", "NEY", "ONB", "SFI", "TAG", "EEM", "UMB", "CRA", "ANU", "SWO",
    "WEV", "EIG", "SSH", "EUS", "EYO", "SIM", "GLE", "ROC", "CIE", "LOG", "REH", "WTH",
    "SLO", "EER", "IFT", "HEV", "FIT", "TUA", "EPU", "GOV", "GOT", "CIN", "INW", "INP",
    "WES", "AGO", "AMS", "SAP", "ITU", "OAC", "OPP", "ETU", "WAT", "SUS", "LAI", "ARN",
    "EEA", "ITC", "ISR", "TCA", "RLD", "ALR", "ASK", "CTU", "LTO", "OGE", "OOT", "EAP",
    "PTI", "EDP", "FEA", "FIL", "TWH", "NGU", "EMS", "VEB", "UIL", "MOT", "ROO", "RPR",
    "RLI", "MPO", "ISB", "ELF", "DFR", "SEI", "OON", "SLE", "MOU", "NEV", "NEC", "CLO",
    "IEL", "LOT", "USA", "APR", "EDC", "RER", "LEI", "FRA", "WEA", "NEN", "ALW", "ESB",
    "BAR", "EYA", "NEI", "NSW", "ELP", "OLA", "EAK", "FEN", "ESN", "RYT", "ORH", "ANB",
    "LSA", "FAR", "GIV", "PIC", "CEP", "TOE", "DPR", "RIM", "EAB", "PAC", "LTI", "HOF",
    "ASU", "SAF", "EWS", "STW", "EEX", "ATM", "MPE", "TBA", "TEE", "INB", "EAV", "DWH",
    "NOM", "MUN", "ENH", "RRO", "PET", "NSP", "MTO", "AGR", "LEW", "RHA", "LVE", "ALB",
    "ODO", "TUN", "LWA", "GUE", "DCA", "RAR", "RWH", "PAI", "HOP", "ROG", "NFR", "FAL",
    "ONN", "INL", "SEP", "BAT", "SMI", "VEM", "CUS", "EPI", "WAL", "DSH", "LAB", "THC",
    "APO", "AAN", "ODI", "FOL", "GOF", "SQU", "SFA", "BUR", "NAC", "RGA", "VIL", "NGD",
    "OCI", "HIR", "RHE", "USS", "EFU", "YWI", "ESM", "YWA", "NPR", "UME", "ODA", "OSA",
    "DRO", "GHE", "GHI", "OPR", "VAN", "LDS", "KAN", "BEG", "RPO", "ORW", "ISD", "DMI",
    "GOI", "FUN", "OAR", "OFR", "ONP", "RIB", "IDI", "RWI", "LOP", "HTO", "EDD", "STP",
    "MAG", "SNE", "SEM", "ERU", "ADT", "BUI", "MMI", "ALK", "GAL", "IBL", "OPU", "ALD",
    "OAL", "YSA", "TSC", "IDT", "PLI", "MUC", "NUM", "SBA", "NEL", "NIA", "ENU", "RUC",
    "LYO", "STM", "BRE", "TOI", "ESD", "DMO", "RYI", "OTS", "SDA", "VOL", "DAM", "ACA",
    "RBE", "PUL", "WON", "EXA", "YED", "LSE", "ACI", "STC", "OFW", "LIM", "ATL", "AIS",
    "FLO", "CEL", "RNO", "CHT", "STY", "BOO", "AUG", "RYS", "UPT", "SEX", "SPL", "RTY",
    "EVA", "YSI", "OFB", "LDR", "UIT", "CLI", "MEI", "IOR", "DOE", "IRA", "LYW", "IGI",
    "SIV", "NAB", "LAW", "TPR", "LDA", "AYO", "RKI", "IDN", "NIO", "IVA", "TYA", "PHO",
    "COA", "ILT", "CHR", "NDG", "PHI", "EFF", "EYW", "RAF", "YIS", "ASR", "NOV", "LYB",
    "AUT", "NAP", "FEE", "NDN", "GFO", "SUB", "SKI", "NTY", "ASF", "OOR", "AFF", "ENN",
    "SDI", "HUR", "STL", "LYC", "DIR", "CUT", "WEN", "AFE", "DNE", "KTH", "PLO", "RAB",
    "NFI", "YMA", "OFL", "UNG", "ANW", "TMO", "ENW", "GAS", "TSS", "JEC", "FOO", "GIS",
    "VAT", "LEY", "TNE", "USH", "RNM", "TSP", "NSC", "GEA", "DIV", "SAD", "DSI", "FCO",
    "OPO", "FHI", "EDL", "YOR", "YCA", "IEV", "YWE", "OMB", "IRD", "BIG", "TIF", "DLI",
    "RHI", "RCA", "GHA", "PTH", "SGO", "YWH", "AVA", "SIB", "GWI", "SUM", "ATP", "LEB",
    "TSU", "CKI", "FEC", "NCR", "RLE", "SME", "CIS", "TDO", "OLU", "KEA", "IEW", "YSE",
    "MOD", "UCE", "FAI", "NIG", "TYE", "TLI", "TSF", "RBA", "DOR", "ABA", "ENB", "BLA",
    "RSC", "THH", "MPI", "ADS", "MMU", "POW", "LFO", "GST", "ASD", "EGU", "SNT", "TNO",
    "ADV", "IET", "BYA", "NVI", "AHA", "OFD", "COV", "POP", "RRA", "NLI", "LTE", "UCA",
    "DWE", "TEI", "NUE", "FLA", "SDO", "RIL", "IRO", "IAM", "TEV", "ESL", "MPR", "EDG",
    "MEW", "ICO", "YAL", "YLE", "NTF", "STB", "LYD", "EFT", "ORB", "HTE", "CEM", "BEN",
    "YAS", "KEE", "NTC", "NTB", "UNE", "NEM", "EOU", "OUC", "PPL", "THU", "TEX", "ICU",
    "RDO", "SEW", "SUL", "OSP", "EXC", "OTB", "GNE", "IDS", "KIL", "LOV", "SYS", "IMS",
    "NIV", "OOM", "STF", "IRC", "OHI", "NMO", "ASL", "ONV", "TDE", "OCE", "EBI", "PTO",
    "LDH", "YPE", "NIE", "NSF", "ELT", "IBE", "FIG", "NPA", "FAT", "LYM", "TYT", "TYP",
    "DNT", "LUE", "URO", "OWH", "OHE", "BLO", "WRI", "HTS", "RKS", "DEO", "HRI", "SOT",
    "LYR", "DUS", "AYB", "CHU", "TFR", "WOM", "YDE", "BIN", "REQ", "GGE", "MEL", "FFO",
    "ARB", "SOP", "UMA", "VIO", "INR", "RTU", "ROR", "TAF", "DAB", "MEE", "LEE", "INJ",
    "MSE", "VEI", "BED", "BIT", "HEK", "URY", "PPR", "TEP", "DAC", "MAC", "ACR", "MMA",
    "ENR", "UTW", "ONR", "NJU", "EYS", "FAS", "KTO", "ORF", "PTE", "CID", "YSH", "VAR",
    "SAW", "EAF", "TLA", "CKA", "SIA", "OBL", "UEN", "RSU", "OKI", "UAT", "UET", "AJO",
    "COS", "CKT", "CHW", "OAS", "LLH", "SBO", "FEW", "MST", "SCU", "BOA", "RRY", "EEV",
    "LMA", "LYF", "VEH", "IDO", "MID", "OIS", "TEO", "SCI", "OWO", "UCK", "DCH", "IRT",
    "OPA", "RSS", "AGU", "RSP", "ULE", "EKI", "BEH", "OYE", "NWE", "DOM", "ADY", "YPR",
    "NKI", "FES", "GIT", "NFE", "SSP", "LOU", "YMO", "URG", "PRA", "DME", "SOW", "EDN",
    "SAV", "ICL", "OPI", "LYP", "KOF", "NLA", "LDT", "RAV", "TFI", "DOT", "RFA", "EAU",
    "NHO", "DBA", "NIM", "CEW", "SKE", "ENF", "RHO", "NNA", "TPA", "WIS", "DFI", "ANH",
    "API", "RPA", "LLC", "IPA", "ADM", "GLA", "TAP", "ABE", "ETY", "GCO", "RMS", "LDN",
    "LAM", "TAM", "YAT", "ANP", "DLA", "IRI", "NBU", "EMU", "NTP", "UNS", "OLS", "DPA",
    "LMO", "GUA", "IRL", "NEF", "RBO", "PAL", "URD", "LYH", "SCE", "ANF", "GNI", "RIP",
    "URP", "IFY", "NLE", "EPH", "EFL", "WOO", "MMO", "IGA", "JOH", "SRA", "ISG", "TIR",
    "TTR", "THM", "YDI", "FEL", "JOR", "DPO", "RTR", "SUE", "YSO", "DHO", "LPR", "RLA",
    "OPT", "KSA", "CTA", "MPT", "YHE", "DUP", "NOL", "OSO", "HEJ", "ALU", "GLI", "GOA",
    "ITL", "CKO", "THW", "BLY", "DSU", "HWA", "TSB", "OFG", "RFI", "OTR", "WNA", "OHN",
    "GIO", "PUR", "MEC", "NFL", "ZED", "ILM", "DBU", "DUN", "OGI", "FLI", "UCC", "NBA",
    "FLE", "RAW", "EIV", "DEB", "EOT", "UNN", "NTM", "EVO", "MIG", "HAI", "LDO", "OAT",
    "NFA", "SSC", "LUS", "NAF", "IWA", "EUP", "CIP", "PAY", "CHS", "ATF", "AMB", "ODY",
    "AGI", "YME", "EXI", "TPE", "RPE", "IMM", "LYE", "TBU", "CTT", "LLP", "LHA", "TDI",
    "ILS", "EUR", "LBU", "TYI", "HUN", "HIE", "GSA", "NIF", "NKS", "WNT", "ATB", "DDO",
    "GEL", "LEP", "MSA", "ENM", "OMS", "YWO", "RYE", "DAP", "OSH", "HCO", "APT", "IRM",
    "MAJ", "AIT", "YFR", "OCR", "GHO", "YLI", "TYS", "ATD", "UPA", "FYO", "NUN", "AFR",
    "AVO", "ADB", "EIM", "USL", "TPO", "LAD", "OWL", "HTT", "BOD", "DWO", "FFA", "LIZ",
    "OCH", "THL", "LAU", "SCL", "ANM", "VOT", "NPO", "VIT", "RDT", "HTI", "DAF", "MOF",
    "IEF", "FIV", "KEL", "POT", "ISU", "OLV", "ENP", "KEY", "RSF", "RLO", "NCY", "RGI",
    "CHN", "UNA", "ILO", "GHS", "ODS", "ONY", "AFO", "MEF", "OUB", "LSI", "THP", "SNA",
    "LOY", "PIR", "TLO", "HTA", "LAG", "YCH", "DTR", "RNS", "CEF", "HUM", "OFN", "LEX",
    "SEF", "NDY", "AMM", "GUI", "CIL", "DPE", "VEC", "HUS", "KST", "LLW", "SGR", "ESR",
    "APH", "UIR", "AWI", "TMI", "TSM", "OCU", "DRU", "JUN", "TEW", "NIZ", "OSU", "UFF",
    "SIX", "QUO", "NKE", "HHI", "DUA", "CHH", "DYO", "EJU", "SIL", "TOK", "IHA", "PHE",
    "WNE", "GSO", "DSP", "DLO", "NSS", "RFR", "EEC", "HMA", "OIT", "DAD", "IPS", "NBO",
    "GOR", "LTA", "RTT", "YNO", "EYC", "TIG", "WNS", "DAV", "RAY", "HHE", "NMI", "UMP",
    "IZA", "NOC", "LUT", "VEO", "TEF", "CCU", "AIM", "KAT", "ALG", "DSC", "OCC", "LDW",
    "IDD", "JOB", "ANR", "NLO", "FIS", "NYT", "RYW", "FHE", "EYH", "UPE", "DOC", "ORU",
    "CTR", "ADU", "NGG", "RSD", "OPS", "TUS", "SYO", "MUL", "OGO", "UPS", "ECK", "FED",
    "RTM", "TEG", "CHM", "SBY", "TGO", "UPO", "TYL", "RBU", "PIE", "CEC", "SUA", "LEL",
    "DEE", "NBY", "REX", "UED", "LCA", "YNE", "COO", "MEB", "REY", "LEH", "CEB", "MBI",
    "NUS", "PME", "GOL", "DIO", "LDC", "UBS", "AIG", "LUM", "LLF", "YLO", "IBU", "RWO",
    "STD", "GNA", "OCT", "LIB", "LME", "CAD", "SEB", "UTU", "EBY", "ASG", "YDO", "GLO",
    "MSO", "AHO", "AYW", "NWO", "EBL", "VIR", "RYC", "OOF", "LSH", "DOI", "UIS", "SOV",
    "HCA", "TAD", "CKL", "LUN", "OOP", "RVA", "GEI", "NUT", "NTD", "GRI", "GUL", "THB",
    "RTW", "RSB", "NUA", "IAS", "UGG", "RCI", "AWH", "EEI", "MEP", "SFU", "HNO", "DEX",
    "ABR", "POF", "GEM", "FMA", "TUT", "SBR", "AUL", "VEP", "AYF", "USP", "DLY", "OPH",
    "KSO", "ENL", "SMU", "ABU", "UMM", "NPE", "SOI", "SEH", "SPU", "XAM", "YBU", "LFA",
    "YCL", "AUN", "PHA", "MBA", "NGN", "REU", "OBS", "OKS", "NAU", "PHY", "WID", "ALH",
    "LLM", "IPP", "ADR", "CHC", "DAG", "BYS", "JAN", "GAB", "USC", "NOP", "DTE", "SOA",
    "AFI", "PPI", "KIS", "BAB", "DVA", "IMT", "DEI", "ISK", "GMA", "EKE", "ETC", "HLE",
    "LYN", "LFI", "EWY", "THF", "GUN", "SYE", "RGO", "RIF", "PES", "XPL", "ATY", "THD",
    "NVO", "LID", "EMT", "NEG", "TSN", "RWE", "DFA", "SFE", "TUP", "NOS", "AFA", "OJE",
    "LOA", "YPA", "LWI", "URB", "ARP", "BON", "RUL", "SRO", "IBI", "DSW", "PIO", "ARW",
    "UTC", "PLY", "WYO", "APS", "EJO", "IGU", "SLY", "DVI", "HST", "RKA", "OUD", "ROJ",
    "NAV", "ALY", "OBI", "DDR", "HOC", "YES", "ASY", "TAX", "ICC", "YBO", "FUR", "PIS",
    "WHY", "DGO", "RPL", "TOV", "SSF", "OFU", "MFO", "HOI", "IRP", "SOO", "TYC", "IGE",
    "GUS", "YPO", "LPA", "SJU", "LYL", "NEP", "NRO", "HOH", "DPL", "OYO", "CCA", "TPL",
    "ITM", "GOU", "EYR", "KON", "XTE", "NSB", "GIC", "MPU", "YLA", "IUM", "AWE", "RYB",
    "FUS", "NDJ", "NSL", "FTO", "OKA", "ZIN", "AHE", "UDG", "NEB", "RUM", "DOL", "OPM",
    "LLR", "ROY", "ITR", "RBI", "KEI", "TBY", "LUB", "IPL", "OIL", "USU", "NCT", "TAY",
    "YSU", "OGY", "BAM", "YHI", "OVA", "HAB", "UPI", "DUE", "COP", "LPE", "EXE", "OWW",
    "NYA", "AEL", "DVE", "ESY", "DUL", "JOI", "YSC", "OEN", "IMO", "CRU", "YGO", "RUG",
    "UIN", "RCU", "TYW", "VEW", "BRU", "HHA", "SVI", "PEE", "FST", "DBO", "ANL", "VEF",
    "NTG", "NSM", "VEG", "JAC", "TSL", "LAP", "SOB", "THY", "OHO", "GPR", "GAG", "ATN",
    "RSM", "ZAT", "EOL", "RFE", "NPL", "MEH", "HSC", "WEH", "IDH", "IAA", "DTA", "NJO",
    "APL", "DBR", "UTF", "ENV", "RGR", "LTU", "HOD", "GFR", "LLN", "NJA", "MBL", "NBR",
    "COT", "DDA", "YSP", "ICR", "ISY", "NEU", "ADC", "ENY", "OBO", "RSQ", "GNO", "SOD",
    "EYD", "SSW", "TEH", "GSI", "LHE", "DAU", "FWH", "ETB", "OMH", "VAI", "EYT", "LLD",
    "NAI", "AUD", "LIP", "YFI", "OMY", "WRO", "HFO", "RSL", "IDG", "EAW", "LSC", "NAW",
    "IRF", "YAC", "ERK", "YIT", "AYN", "TGE", "OGA", "YBA", "DGR", "ONJ", "RYF", "RYP",
    "EMY", "XIS", "HME", "TSR", "UAN", "ITB", "PST", "NYS", "LCH", "BAD", "SOS", "BOY",
    "OOS", "VOR", "OIC", "UEL", "AHI", "TFA", "EWR", "OBU", "ELV", "EOV", "TBO", "FLU",
    "CEH", "SSM", "DJU", "RIZ", "IFO", "TAU", "AYH", "CEE", "URF", "WAI", "UTM", "USO",
    "XTR", "STG", "URV", "SVE", "BUL", "TSD", "ACL", "YHO", "EEF", "MOC", "TDA", "IPT",
    "LTS", "PAP", "GIR", "USB", "VEY", "UIC", "EKN", "YET", "PSA", "LBA", "RGU", "FSE",
    "IOD", "OWD", "WEB", "DAI", "DIM", "SAU", "YAF", "AZI", "IGG", "NOO", "ITF", "TGA",
    "ULI", "LTR", "OSC", "LNE", "CHD", "XCE", "OUA", "EAI", "STN", "IOL", "GWA", "RCR",
    "GUR", "UTR", "LAL", "EKS", "AYM", "SOH", "LLU", "JUL",
];
