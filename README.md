# OpenWeather API Automation Testing - South Jakarta

This project is an automated API testing suite for OpenWeather, covering 5-day weather forecasts and air pollution data in the South Jakarta area. Built using **Katalon Studio**.

## How to Run the Project

1. **Prerequisites:**
   - Download and install [Katalon Studio](https://katalon.com/download).
   - Sign up at [OpenWeatherMap](https://openweathermap.org/) to get your API Key.

2. **Setup:**
   - Clone this repository: `git clone https://github.com/Delonix1Regia/api-testing-south-jakarta-weather.git`
   - Open Katalon Studio and select **Open Project**.
   - Navigate to the cloned folder.

3. **Configure API Key:**
   - Go to **Profiles > default**.
   - Change the value of `APIKey` with your actual OpenWeather API Key.

4. **Execution:**
   - Open **Test Suites > TS_OpenWeather**.
   - Click the **Run** button (Play icon) and choose your preferred browser/engine.

## How to Get the Report

After the execution is finished:
- Go to the **Reports** folder in the Katalon Test Explorer.
- Right-click on the latest generated report.
- Select **Export as > HTML**.
- Alternatively, you can find the raw report files in the `Reports/` directory within the project folder.

## Project Structure

- **Object Repository**: Contains API Request objects (Endpoints, Query Parameters, and Authorization).
- **Test Cases**: Contains individual test logic (Positive and Negative scenarios).
- **Test Suites**: A collection of test cases to be executed together.
- **Profiles**: Contains environment variables (Global Variables) such as `APIKey`.
- **Scripts**: Groovy scripts that define the test execution steps.

## Test Scenarios Covered
- **TC01**: Success Get 5-Day Forecast (Verify 200 OK & JSON Structure).
- **TC02**: Success Get Air Pollution Data.
- **TC03**: Failed Get Data with Invalid API Key (Verify 401 Unauthorized).
- **TC04**: Failed Get Data with Invalid Coordinates (Verify 400 Bad Request).