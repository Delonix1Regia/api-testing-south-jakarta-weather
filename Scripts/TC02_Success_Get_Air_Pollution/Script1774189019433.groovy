import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper

def response = WS.sendRequest(findTestObject('Object Repository/weather forecast/get_air_polution'))

WS.verifyResponseStatusCode(response, 200)

def json = new JsonSlurper().parseText(response.getResponseText())
int aqi = json.list[0].main.aqi
assert aqi >= 1 && aqi <= 5

WS.verifyElementPropertyValue(response, 'coord.lat', -6.2615)
WS.verifyElementPropertyValue(response, 'coord.lon', 106.8106)

WS.validateJsonAgainstSchema(response, '{"type": "object", "required": ["coord", "list"]}')