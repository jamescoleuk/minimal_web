CREATE TABLE IF NOT EXISTS forecast (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    forecastType TEXT NOT NULL,
    data JSON
);

-- CREATE TABLE IF NOT EXISTS dateForecast (
--     id INTEGER PRIMARY KEY NOT NULL,
--     forecastId INTEGER NOT NULL,
--     startDate DATE NOT NULL,
--     endDate DATE NOT NULL,
--     FOREIGN KEY (forecastId) REFERENCES forecast(id)
-- );
-- CREATE TABLE IF NOT EXISTS dateRange (
--     id INTEGER PRIMARY KEY NOT NULL,
--     startDate DATE NOT NULL,
--     endDate DATE NOT NULL,
--     value INTEGER NOT NULL,
--     dateForecastId INTEGER NOT NULL,
--     FOREIGN KEY(dateForecastId) REFERENCES dateForecast(id)
-- );