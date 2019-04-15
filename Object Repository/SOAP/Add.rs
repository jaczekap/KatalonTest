<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add</name>
   <tag></tag>
   <elementGuidId>8bc5fab4-4d99-49ea-b10b-e71b3de37126</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;?xml version=&quot;1.0&quot; encoding=&quot;UTF-8&quot;?>&lt;SOAP-ENV:Envelope xmlns:SOAP-ENV=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tns=&quot;http://tempuri.org/&quot;>
  &lt;SOAP-ENV:Header/>
  &lt;SOAP-ENV:Body>
    &lt;tns:Add>
      &lt;tns:intA>5&lt;/tns:intA>
      &lt;tns:intB>6&lt;/tns:intB>
    &lt;/tns:Add>
  &lt;/SOAP-ENV:Body>
&lt;/SOAP-ENV:Envelope>
</soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>Add</soapServiceFunction>
   <variables>
      <defaultValue>'2'</defaultValue>
      <description></description>
      <id>559cd550-e9b9-4eca-bdf0-a02a40912b3f</id>
      <masked>false</masked>
      <name>number1</name>
   </variables>
   <variables>
      <defaultValue>'9'</defaultValue>
      <description></description>
      <id>fee09931-799c-4c60-b2fa-6b9e78343668</id>
      <masked>false</masked>
      <name>number2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()





WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))


assertThat(response.getResponseText()).contains('AddResult')</verificationScript>
   <wsdlAddress>http://www.dneonline.com/calculator.asmx?wsdl</wsdlAddress>
</WebServiceRequestEntity>
