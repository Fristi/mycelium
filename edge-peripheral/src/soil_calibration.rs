const DRY_PF: f32 = 300.0;
const WET_PF: f32 = 900.0;

pub fn pf_to_percent(pf: f32) -> u8 {
    let pct = (pf - DRY_PF) / (WET_PF - DRY_PF) * 100.0;
    pct.clamp(0.0, 100.0) as u8
}
