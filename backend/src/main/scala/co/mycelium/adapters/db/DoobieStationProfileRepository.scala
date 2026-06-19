package co.mycelium.adapters.db

import co.mycelium.domain.{PlantProfile, StationPlantProfile}
import co.mycelium.ports.StationProfileRepository
import doobie.*
import doobie.implicits.*
import doobie.postgres.implicits.*

import java.util.UUID

object DoobieStationProfileRepository extends StationProfileRepository[ConnectionIO] {

  override def upsert(profile: PlantProfile, stationId: UUID): ConnectionIO[Int] =
    sql"""
      INSERT INTO station_profile (
        station_id, name,
        light_mmol_start, light_mmol_end,
        light_lux_start, light_lux_end,
        temperature_start, temperature_end,
        humidity_start, humidity_end,
        soil_moisture_start, soil_moisture_end,
        soil_ec_start, soil_ec_end
      ) VALUES (
        $stationId, ${profile.name},
        ${profile.variables.lightMmol.start}, ${profile.variables.lightMmol.end},
        ${profile.variables.lightLux.start}, ${profile.variables.lightLux.end},
        ${profile.variables.temperature.start}, ${profile.variables.temperature.end},
        ${profile.variables.humidity.start}, ${profile.variables.humidity.end},
        ${profile.variables.soilMoisture.start}, ${profile.variables.soilMoisture.end},
        ${profile.variables.soilEc.start}, ${profile.variables.soilEc.end}
      )
      ON CONFLICT (station_id) DO UPDATE SET
        name = EXCLUDED.name,
        light_mmol_start = EXCLUDED.light_mmol_start,
        light_mmol_end = EXCLUDED.light_mmol_end,
        light_lux_start = EXCLUDED.light_lux_start,
        light_lux_end = EXCLUDED.light_lux_end,
        temperature_start = EXCLUDED.temperature_start,
        temperature_end = EXCLUDED.temperature_end,
        humidity_start = EXCLUDED.humidity_start,
        humidity_end = EXCLUDED.humidity_end,
        soil_moisture_start = EXCLUDED.soil_moisture_start,
        soil_moisture_end = EXCLUDED.soil_moisture_end,
        soil_ec_start = EXCLUDED.soil_ec_start,
        soil_ec_end = EXCLUDED.soil_ec_end
    """.update.run

  override def getPlantProfilesByUserId(userId: String): ConnectionIO[List[StationPlantProfile]] =
    sql"""
      SELECT s.id, s.mac_addr, sp.name,
             sp.light_mmol_start, sp.light_mmol_end,
             sp.light_lux_start, sp.light_lux_end,
             sp.temperature_start, sp.temperature_end,
             sp.humidity_start, sp.humidity_end,
             sp.soil_moisture_start, sp.soil_moisture_end,
             sp.soil_ec_start, sp.soil_ec_end
      FROM station_profile sp
      INNER JOIN stations s ON s.id = sp.station_id
      WHERE s.user_id = $userId
    """.query[StationPlantProfile].to[List]

  override def getByStationId(stationId: UUID): ConnectionIO[Option[PlantProfile]] =
    sql"""
      SELECT sp.name,
             sp.light_mmol_start, sp.light_mmol_end,
             sp.light_lux_start, sp.light_lux_end,
             sp.temperature_start, sp.temperature_end,
             sp.humidity_start, sp.humidity_end,
             sp.soil_moisture_start, sp.soil_moisture_end,
             sp.soil_ec_start, sp.soil_ec_end
      FROM station_profile sp
      WHERE sp.station_id = $stationId
    """.query[PlantProfile].option
}
