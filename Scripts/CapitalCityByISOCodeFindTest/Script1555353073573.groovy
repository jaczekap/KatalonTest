import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import static org.assertj.core.api.Assertions.*

response = WS.sendRequest(findTestObject('CountryList/List_Of_Countries'))

String respondedXml = response.responseBodyContent

def dataValue = new XmlSlurper().parseText(respondedXml)

String[] codesReturned = new String[2]

String[] capitalsCityTestData = new String[2]

(capitalsCityTestData[0]) = 'Kabul'

(capitalsCityTestData[1]) = 'Tirana'

for (int i = 0; i < 2; i++) {
    (codesReturned[i]) = dataValue.ListOfCountryNamesByNameResult.tCountryCodeAndName[(i + 1)].sISOCode.text()
}

for (int i = 0; i < 2; i++) {
    GlobalVariable.ISOCode = (codesReturned[i])

    response1 = WS.sendRequest(findTestObject('CountryList/CountryCapital'))

    WS.verifyElementText(response1, 'CapitalCityResponse.CapitalCityResult', capitalsCityTestData[i])
}