import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

def response = WS.sendRequest(findTestObject('Object Repository/weather forecast/get_weather_invalid_coor'))

WS.verifyResponseStatusCode(response, 400)


WS.verifyElementPropertyValue(response, 'message', 'wrong latitude')