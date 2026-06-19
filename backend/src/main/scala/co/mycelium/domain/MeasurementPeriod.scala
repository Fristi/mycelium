package co.mycelium.domain

sealed abstract class MeasurementPeriod(val repr: String)

object MeasurementPeriod {
  case object LastTwentyFourHours extends MeasurementPeriod("last-24-hours")
  case object LastSevenDays       extends MeasurementPeriod("last-7-days")
  case object LastTwoWeeks        extends MeasurementPeriod("last-2-weeks")
  case object LastMonth           extends MeasurementPeriod("last-month")

  val all = Set(LastTwentyFourHours, LastSevenDays, LastTwoWeeks, LastMonth)

  def fromString(str: String): Option[MeasurementPeriod] = all.find(_.repr == str)

  /** Duration of each aggregated measurement bucket for this period. */
  def bucketDuration(period: MeasurementPeriod): java.time.Duration = period match
    case LastTwentyFourHours => java.time.Duration.ofMinutes(15)
    case LastSevenDays       => java.time.Duration.ofDays(1)
    case LastTwoWeeks        => java.time.Duration.ofDays(1)
    case LastMonth           => java.time.Duration.ofDays(1)

  /** PostgreSQL interval literal for filtering events within the period window. */
  def lookbackInterval(period: MeasurementPeriod): String = period match
    case LastTwentyFourHours => "24 hours"
    case LastSevenDays       => "7 days"
    case LastTwoWeeks        => "14 days"
    case LastMonth           => "31 days"
}
