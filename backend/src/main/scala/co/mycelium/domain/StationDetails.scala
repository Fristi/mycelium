package co.mycelium.domain

import io.circe.{Decoder, Encoder}

final case class StationDetails(
    station: Station,
    measurements: List[StationMeasurement],
    waterings: List[StationWatering] = Nil,
    growthPeriods: List[GrowthPeriod] = Nil
) derives Encoder.AsObject,
      Decoder
