import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

def response = WS.sendRequest(findTestObject('Object Repository/weather forecast/get_weather_invalid_key'))

WS.verifyResponseStatusCode(response, 401)

WS.verifyElementPropertyValue(response, 'message', 'Invalid API key. Please see https://openweathermap.org/faq#error401 for more info.')