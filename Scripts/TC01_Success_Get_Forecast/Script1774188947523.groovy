import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper

def response = WS.sendRequest(findTestObject('Object Repository/weather forecast/get_5_days_forecast'))

WS.verifyResponseStatusCode(response, 200)
def json = new JsonSlurper().parseText(response.getResponseText())
assert json.list.size() == 40

WS.validateJsonAgainstSchema(response, '{"type": "object", "required": ["list", "city"]}')