package co.mycelium.service

import co.mycelium.domain.*

/** Classifies measurement buckets into productive / non-productive growth periods.
  *
  * Soil moisture is excluded: profile ranges use % but measurements use soil pF.
  * Non-productive reason priority when multiple violations: heat > light > humidity.
  * Daily buckets are expected to carry peak lux and peak temperature (see repository).
  */
object GrowthPeriodClassifier {

  def classify(
      measurements: List[StationMeasurement],
      profile: Option[PlantProfile],
      period: MeasurementPeriod
  ): List[GrowthPeriod] =
    profile match
      case None => Nil
      case Some(p) if measurements.isEmpty => Nil
      case Some(p) =>
        val bucketDuration = MeasurementPeriod.bucketDuration(period)
        val classified = measurements.map(m => (m, classifyBucket(m, p.variables)))
        mergeConsecutive(classified, bucketDuration)

  private def classifyBucket(
      measurement: StationMeasurement,
      variables: PlantProfileVariables
  ): GrowthPeriodKind =
    nonProductiveReason(measurement, variables) match
      case Some(reason) => GrowthPeriodKind.NonProductive(reason)
      case None         => GrowthPeriodKind.Productive

  private def nonProductiveReason(
      m: StationMeasurement,
      v: PlantProfileVariables
  ): Option[NonProductiveReason] =
    if m.temperature > v.temperature.end then Some(NonProductiveReason.HeatStress)
    else if m.lux < v.lightLux.start then Some(NonProductiveReason.LowLight)
    else if m.lux > v.lightLux.end then Some(NonProductiveReason.HighLight)
    else if m.humidity < v.humidity.start then Some(NonProductiveReason.LowHumidity)
    else if m.humidity > v.humidity.end then Some(NonProductiveReason.HighHumidity)
    else None

  private def mergeConsecutive(
      classified: List[(StationMeasurement, GrowthPeriodKind)],
      bucketDuration: java.time.Duration
  ): List[GrowthPeriod] =
    if classified.isEmpty then Nil
    else
      val segments = classified.zipWithIndex.map { case ((m, kind), idx) =>
        val end =
          if idx == classified.length - 1 then m.on.plus(bucketDuration)
          else classified(idx + 1)._1.on
        (m.on, end, kind)
      }

      segments
        .foldLeft(List.empty[GrowthPeriod]) {
          case (acc, (start, end, kind)) =>
            acc match
              case prev :: rest if sameKind(prev.kind, kind) =>
                GrowthPeriod(prev.start, end, prev.kind) :: rest
              case _ =>
                GrowthPeriod(start, end, kind) :: acc
        }
        .reverse

  private def sameKind(a: GrowthPeriodKind, b: GrowthPeriodKind): Boolean = (a, b) match
    case (GrowthPeriodKind.Productive, GrowthPeriodKind.Productive) => true
    case (GrowthPeriodKind.NonProductive(r1), GrowthPeriodKind.NonProductive(r2)) => r1 == r2
    case _ => false
}
