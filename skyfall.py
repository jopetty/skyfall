def get_sidereal_time(time: float, date: (int, int, int), longitude: float) -> float:
  year, month, day = date

  # Calculate the Julian Day
  A = int(year/100)
  B = 2 - A + int(A/4)
  jd = int(365.25*(year + 4716)) + int(30.6001*(month + 1)) + day + B - 1524.5

  # Calculate Greenwich Sidereal Time
  T = (jd + time/24.0 - 2451545.0)/36525.0
  qo = 280.46061837 + 360.98564736629 * (jd -2451545.0) + 0.000387933 * T**2 - T**3/38710000

  # Calculate Local Sidereal Time
  q = qo + longitude

  return q

def get_local_hour_angle(lst: float, right_ascension: float) -> float:
  return lst - right_ascension

def get_coordinates(latitude: float, declination: float, lha: float) -> (float, float):
  pass

if __name__ == "__main__":
  currtime = 23.87778
  currdate = (2020, 3, 8)
  currlong = -114.093810
  ljd = get_sidereal_time(currtime, currdate, currlong)

  print(f"Local Sidereal Time: {ljd}")