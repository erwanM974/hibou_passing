use std::fmt;
use crate::process::ana::verdict::inconc::InconcReason;

pub enum AnalysisLocalVerdict{
    Cov,
    GloPref,
    MultiPref,
    Slice,
    Inconc(InconcReason),
    Out(bool),   // bool for if it's known via local analysis
}

impl fmt::Display for AnalysisLocalVerdict {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalysisLocalVerdict::Cov => {
                write!(f,"Cov")
            },
            AnalysisLocalVerdict::GloPref => {
                write!(f,"GloPref")
            },
            AnalysisLocalVerdict::MultiPref => {
                write!(f,"MultiPref")
            },
            AnalysisLocalVerdict::Slice => {
                write!(f,"Slice")
            },
            AnalysisLocalVerdict::Inconc(reason) => {
                write!(f,"Inconc {:}", reason)
            },
            AnalysisLocalVerdict::Out(ref loc) => {
                if *loc {
                    write!(f,"Out-l")
                } else {
                    write!(f,"Out")
                }
            }
        }
    }

}
