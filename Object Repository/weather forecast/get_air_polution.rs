<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>get_air_polution</name>
   <tag></tag>
   <elementGuidId>8e30576f-c6a8-4bec-91ef-251d34748847</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>10.4.3</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://api.openweathermap.org/data/2.5/air_pollution?lat=-6.2615&amp;lon=106.8106&amp;appid=${GlobalVariable.APIKey}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

String airPollutionSchema = &quot;&quot;&quot;
{
  &quot;type&quot;: &quot;object&quot;,
  &quot;required&quot;: [&quot;coord&quot;, &quot;list&quot;],
  &quot;properties&quot;: {
    &quot;list&quot;: { &quot;type&quot;: &quot;array&quot; }
  }
}
&quot;&quot;&quot;
WS.validateJsonAgainstSchema(response, airPollutionSchema)
WS.verifyElementPropertyValue(response, 'list[0].main.aqi', 1)

def json = new groovy.json.JsonSlurper().parseText(response.getResponseText())
assert json.list[0].main.aqi >= 1 &amp;&amp; json.list[0].main.aqi &lt;= 5
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
