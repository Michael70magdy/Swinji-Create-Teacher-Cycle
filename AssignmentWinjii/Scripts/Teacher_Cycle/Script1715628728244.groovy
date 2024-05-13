import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl('https://swinji.azurewebsites.net/account/login')

WebUI.setText(findTestObject('Object Repository/Page_Log in - WinjiGo/Email_TextBox'), 'adminmomen@mailinator.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Log in - WinjiGo/Password_TextBox'), '+uAi5TIr8h2LixnDdUIaKg==')

WebUI.click(findTestObject('Object Repository/Page_Log in - WinjiGo/Login_Button'))

WebUI.click(findTestObject('Object Repository/Page_LMS Now - WinjiGo/AddministrationTools_Tab'))

WebUI.click(findTestObject('Object Repository/Page_LMS Now - WinjiGo/Administration_Button'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/ParentSchool_Tab_Select'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/AddUsers_Button'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/CreateNewUser_Button'))

int rn

rn = ((Math.random() * 10000) as int)

WebUI.setText(findTestObject('Object Repository/Page_Adminstration - WinjiGo/Create__FirstName_TextField'), 'Michael' + 
    rn.toString())

UserName = WebUI.getText(findTestObject('Object Repository/Page_Adminstration - WinjiGo/Create__FirstName_TextField'))

WebUI.setText(findTestObject('Object Repository/Page_Adminstration - WinjiGo/CreateUser__LastName_TextField'), 'Teacher')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Adminstration - WinjiGo/Select_Language_DropDown'), 'string:en', 
    true)

WebUI.setText(findTestObject('Object Repository/Page_Adminstration - WinjiGo/CreateUser__Email_TextField'), ('Michael' + 
    rn.toString()) + '@gmail.com')

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/DotSelect_CreatePassword_Select'))

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Adminstration - WinjiGo/CreateUser_SetPassword_TextField'), 
    '+uAi5TIr8h2LixnDdUIaKg==')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Adminstration - WinjiGo/CreateUser_ReEnterPassword_TextField'), 
    '+uAi5TIr8h2LixnDdUIaKg==')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Adminstration - WinjiGo/CreateUser_SelectTeacherRule_DropDown'), 
    'number:1', true)

WebUI.scrollToElement(findTestObject('Object Repository/Page_Adminstration - WinjiGo/CreateUser_SelectSubject_DropDown'), 
    0)

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/CreateUser_SelectSubject_DropDown'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/ChooseSubject_DropDown'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/CreateUser_CloseSubjectDropDown_ArrowButton'))

WebUI.setText(findTestObject('Object Repository/Page_Adminstration - WinjiGo/CreateUser_TextNote_TextField'), 'This is Test User As A teacher ')

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/CreateUser_Save_Button'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/Filter_Button'))

WebUI.setText(findTestObject('Object Repository/Page_Adminstration - WinjiGo/SearchUser_SearchField'), 'Michael' + rn.toString())

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/SearchTab_Button'))

WebUI.verifyElementText(findTestObject('Object Repository/Page_Adminstration - WinjiGo/TeacherOption_DropDown'), 'Teacher', 
    FailureHandling.CONTINUE_ON_FAILURE)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Adminstration - WinjiGo/SelectTeacherRole_DropDown'), '1', 
    true)

WebUI.verifyElementText(findTestObject('Object Repository/Page_Adminstration - WinjiGo/TeacherOption_DropDown'), 'Teacher', 
    FailureHandling.CONTINUE_ON_FAILURE)

WebUI.setText(findTestObject('Object Repository/Page_Adminstration - WinjiGo/SearchUser_SearchField'), ('Michael' + rn.toString()) + 
    '@gmail.com')

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/Search_Button'))

WebUI.verifyElementText(findTestObject('Object Repository/Page_Adminstration - WinjiGo/TeacherOption_DropDown'), 'Teacher', 
    FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/EditTeacher_DropDown'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/Edit_User_Button'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/EditUser_AddRule_DropDown'))

WebUI.verifyElementText(findTestObject('Object Repository/Page_Adminstration - WinjiGo/VerifyEnglishSubject_Text'), 'English', 
    FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/EditUser_NewSchool_DropDown'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/EditUser_ChooseNewSchool_FromDropDown'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/EditUser_SelectNewRole_DropDown'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/Co_teacher_Role_select'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Adminstration - WinjiGo/EditUser_SelectSubject_DropDown'), 
    0)

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/EditUser_SelectSubject_DropDown'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/ChooseSubject_DropDown'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/CreateUser_CloseSubjectDropDown_ArrowButton'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/EditUser_SaveNewInformation_Button'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/EditUser_SaveAllUserInformation_Button'))

WebUI.click(findTestObject('Object Repository/Page_LMS Now - WinjiGo/AddministrationTools_Tab'))

WebUI.click(findTestObject('Object Repository/Page_LMS Now - WinjiGo/Administration_Button'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/ParentSchool_Tab_Select'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/Filter_Button'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Adminstration - WinjiGo/UserSearch_Select_Co-TeacherRole_DropDown'), 
    '708', true)

WebUI.setText(findTestObject('Object Repository/Page_Adminstration - WinjiGo/SearchUser_SearchField'), ('Michael' + rn.toString()) + 
    '@gmail.com')

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/SearchTab_Button'))

WebUI.verifyElementText(findTestObject('Object Repository/Page_Adminstration - WinjiGo/Verify_Co_Teacher_Text'), 'Co-Teacher', 
    FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/UserName_Click'))

WebUI.verifyElementText(findTestObject('Object Repository/Page_Profile - WinjiGo/Verify_Teacher_EnglishSubject_Text'), 'English')

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/Admin_Menu'))

WebUI.click(findTestObject('Object Repository/Page_Adminstration - WinjiGo/LogOut_Button'))

WebUI.closeBrowser()

