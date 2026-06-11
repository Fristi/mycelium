package co.mycelium.adapters.db

import co.mycelium.ports.{
  Repositories,
  StationMeasurementRepository,
  StationProfileRepository,
  StationRepository,
  StationWateringRepository
}
import doobie.ConnectionIO

object DoobieRepositories extends Repositories[ConnectionIO] {
  override def waterings: StationWateringRepository[ConnectionIO] =
    DoobieStationWateringRepository
  override def stations: StationRepository[ConnectionIO] =
    DoobieStationRepository
  override def measurements: StationMeasurementRepository[ConnectionIO] =
    DoobieStationMeasurementRepository
  override def stationProfile: StationProfileRepository[ConnectionIO] =
    DoobieStationProfileRepository
}
