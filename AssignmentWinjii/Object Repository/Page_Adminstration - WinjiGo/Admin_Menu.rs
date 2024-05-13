<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>Admin_Menu</name>
   <tag></tag>
   <elementGuidId>b8da77aa-54ec-454e-89c7-03a4871a87fe</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>li.lms-height-auto.text-align-center.lms-padding-end-0.lms-margin-bottom-0.lms-border-none.lms-user-menu.lms-icon-visible</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//aside[@id='left-panel']/div/nav/ul/li</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>li</value>
      <webElementGuid>21c10823-00b6-4a7b-842b-f42783236335</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>lms-height-auto text-align-center lms-padding-end-0 lms-margin-bottom-0 lms-border-none lms-user-menu lms-icon-visible</value>
      <webElementGuid>04edf6de-3332-437f-bc40-5006506066ce</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                
                    
                        
                    
                
                
                    T.Moomen Admin
                
                        Admin
                
                            

            
                    
                        
                        
                                Admin
                    

                



        



    //$(&quot;.lms-user-name&quot;).each(function () {

    //    smartTrimText(this, 17);

    //});

    function smartTrimText(elem, numChars) {
        var text, trimmedStr, afterLastChar, dots, altCharIndex, isSmartTrim;
        text = $.trim($(elem).text());
        trimmedStr = text.substring(0, numChars - 1); //the trimmed text according to the number of characters required
        afterLastChar = text.charAt(numChars - 1); //the character after the last one
        dots = &quot;&lt;span class='dots'>...&lt;/span>&quot;;

        if (afterLastChar === &quot; &quot;) isSmartTrim = false; //will no require smart trimming, just normal trimming
        else isSmartTrim = true;

        if (text.length > numChars) {
            if (isSmartTrim) {
                altCharIndex = trimmedStr.lastIndexOf(&quot; &quot;);
                trimmedStr = trimmedStr.substring(0, altCharIndex);
            }
            $(elem).text(trimmedStr).append(dots);
        }
    }



                
                
                    

    T.Moomen Admin
            Admin





        
            
                
                Switch Role
            
            
                    

                        


                                
                                    
                                        
                                            Admin
                                        
                                            
Organization                                            
                                    
                                    
                                
                        

                    
                    

                        


                                
                                    Teacher
                                
                                
Primary School
                                
                        

                    

            
        



    

        
            
             Change Password
        
    

    
        
        Settings
    



    
        
         Log out
    




        function winji_logOff() {
                
                    var hdn = document.getElementById('logOffExternalProvider');
                    hdn.value = &quot;false&quot;;
                
            if (window.eduWorx.config.enableChat &amp;&amp; document.getElementById('cc-app')) { // check if chat sdk enabled
            window.eduWorx.logoutFromChatSdk();   //// log out from chatSdk
            }

            removeUserSchoolPermissionsFromSessionStorage();

            document.getElementById('logoutForm').submit();
        }

        function removeUserSchoolPermissionsFromSessionStorage(){
            if (window?.eduWorx.currentUser.Id &amp;&amp; window?.eduWorx.currentUser.role){
                var key = window.eduWorx.currentUser.Id + '_' + window.eduWorx.currentUser.role + '_' + 'SchoolPermissions';
                sessionStorage.removeItem(key);
            }
        }

                eduWorx.currentUser.isLocalAccount = true;

         if (typeof angular != 'undefined') {
             angular.element(document).ready(function () {
                 var showMoreRolesDiv = document.getElementById(&quot;showMoreRolesDiv&quot;);
                 if (showMoreRolesDiv) {
                     angular.module('app.schoolSelector');
                     try {
                         angular.bootstrap(showMoreRolesDiv, [&quot;app.schoolSelector&quot;]);
                     } catch (e) {

                     }

                 }
             });
         }


                
            </value>
      <webElementGuid>ec0af146-90ae-4eda-b9df-042c60d4532d</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;left-panel&quot;)/div[@class=&quot;lms-left-panel-bg&quot;]/nav[@class=&quot;lms-left-nav lms-padding-bottom-10&quot;]/ul[@class=&quot;lms-user-profile lms-animate-ul-text lms-loading-ico&quot;]/li[@class=&quot;lms-height-auto text-align-center lms-padding-end-0 lms-margin-bottom-0 lms-border-none lms-user-menu lms-icon-visible&quot;]</value>
      <webElementGuid>36330c6f-d506-40d6-8a5a-3457e5a6fc71</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//aside[@id='left-panel']/div/nav/ul/li</value>
      <webElementGuid>d13fbd5f-c84a-48e8-a48e-f495975ff9db</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Collapse/Expand Menu'])[1]/following::li[1]</value>
      <webElementGuid>1886ea3b-c51d-4644-8fee-2224bb441b79</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Skip to Content (Press Enter)'])[1]/following::li[1]</value>
      <webElementGuid>0e005f19-0864-4e34-bf93-21f1e2333bdf</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//li</value>
      <webElementGuid>cb26a206-07bc-40cb-9fb5-7ec86a6775f5</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//li[(text() = concat(&quot;
                
                    
                        
                    
                
                
                    T.Moomen Admin
                
                        Admin
                
                            

            
                    
                        
                        
                                Admin
                    

                



        



    //$(&quot;.lms-user-name&quot;).each(function () {

    //    smartTrimText(this, 17);

    //});

    function smartTrimText(elem, numChars) {
        var text, trimmedStr, afterLastChar, dots, altCharIndex, isSmartTrim;
        text = $.trim($(elem).text());
        trimmedStr = text.substring(0, numChars - 1); //the trimmed text according to the number of characters required
        afterLastChar = text.charAt(numChars - 1); //the character after the last one
        dots = &quot;&lt;span class=&quot; , &quot;'&quot; , &quot;dots&quot; , &quot;'&quot; , &quot;>...&lt;/span>&quot;;

        if (afterLastChar === &quot; &quot;) isSmartTrim = false; //will no require smart trimming, just normal trimming
        else isSmartTrim = true;

        if (text.length > numChars) {
            if (isSmartTrim) {
                altCharIndex = trimmedStr.lastIndexOf(&quot; &quot;);
                trimmedStr = trimmedStr.substring(0, altCharIndex);
            }
            $(elem).text(trimmedStr).append(dots);
        }
    }



                
                
                    

    T.Moomen Admin
            Admin





        
            
                
                Switch Role
            
            
                    

                        


                                
                                    
                                        
                                            Admin
                                        
                                            
Organization                                            
                                    
                                    
                                
                        

                    
                    

                        


                                
                                    Teacher
                                
                                
Primary School
                                
                        

                    

            
        



    

        
            
             Change Password
        
    

    
        
        Settings
    



    
        
         Log out
    




        function winji_logOff() {
                
                    var hdn = document.getElementById(&quot; , &quot;'&quot; , &quot;logOffExternalProvider&quot; , &quot;'&quot; , &quot;);
                    hdn.value = &quot;false&quot;;
                
            if (window.eduWorx.config.enableChat &amp;&amp; document.getElementById(&quot; , &quot;'&quot; , &quot;cc-app&quot; , &quot;'&quot; , &quot;)) { // check if chat sdk enabled
            window.eduWorx.logoutFromChatSdk();   //// log out from chatSdk
            }

            removeUserSchoolPermissionsFromSessionStorage();

            document.getElementById(&quot; , &quot;'&quot; , &quot;logoutForm&quot; , &quot;'&quot; , &quot;).submit();
        }

        function removeUserSchoolPermissionsFromSessionStorage(){
            if (window?.eduWorx.currentUser.Id &amp;&amp; window?.eduWorx.currentUser.role){
                var key = window.eduWorx.currentUser.Id + &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; + window.eduWorx.currentUser.role + &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;SchoolPermissions&quot; , &quot;'&quot; , &quot;;
                sessionStorage.removeItem(key);
            }
        }

                eduWorx.currentUser.isLocalAccount = true;

         if (typeof angular != &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
             angular.element(document).ready(function () {
                 var showMoreRolesDiv = document.getElementById(&quot;showMoreRolesDiv&quot;);
                 if (showMoreRolesDiv) {
                     angular.module(&quot; , &quot;'&quot; , &quot;app.schoolSelector&quot; , &quot;'&quot; , &quot;);
                     try {
                         angular.bootstrap(showMoreRolesDiv, [&quot;app.schoolSelector&quot;]);
                     } catch (e) {

                     }

                 }
             });
         }


                
            &quot;) or . = concat(&quot;
                
                    
                        
                    
                
                
                    T.Moomen Admin
                
                        Admin
                
                            

            
                    
                        
                        
                                Admin
                    

                



        



    //$(&quot;.lms-user-name&quot;).each(function () {

    //    smartTrimText(this, 17);

    //});

    function smartTrimText(elem, numChars) {
        var text, trimmedStr, afterLastChar, dots, altCharIndex, isSmartTrim;
        text = $.trim($(elem).text());
        trimmedStr = text.substring(0, numChars - 1); //the trimmed text according to the number of characters required
        afterLastChar = text.charAt(numChars - 1); //the character after the last one
        dots = &quot;&lt;span class=&quot; , &quot;'&quot; , &quot;dots&quot; , &quot;'&quot; , &quot;>...&lt;/span>&quot;;

        if (afterLastChar === &quot; &quot;) isSmartTrim = false; //will no require smart trimming, just normal trimming
        else isSmartTrim = true;

        if (text.length > numChars) {
            if (isSmartTrim) {
                altCharIndex = trimmedStr.lastIndexOf(&quot; &quot;);
                trimmedStr = trimmedStr.substring(0, altCharIndex);
            }
            $(elem).text(trimmedStr).append(dots);
        }
    }



                
                
                    

    T.Moomen Admin
            Admin





        
            
                
                Switch Role
            
            
                    

                        


                                
                                    
                                        
                                            Admin
                                        
                                            
Organization                                            
                                    
                                    
                                
                        

                    
                    

                        


                                
                                    Teacher
                                
                                
Primary School
                                
                        

                    

            
        



    

        
            
             Change Password
        
    

    
        
        Settings
    



    
        
         Log out
    




        function winji_logOff() {
                
                    var hdn = document.getElementById(&quot; , &quot;'&quot; , &quot;logOffExternalProvider&quot; , &quot;'&quot; , &quot;);
                    hdn.value = &quot;false&quot;;
                
            if (window.eduWorx.config.enableChat &amp;&amp; document.getElementById(&quot; , &quot;'&quot; , &quot;cc-app&quot; , &quot;'&quot; , &quot;)) { // check if chat sdk enabled
            window.eduWorx.logoutFromChatSdk();   //// log out from chatSdk
            }

            removeUserSchoolPermissionsFromSessionStorage();

            document.getElementById(&quot; , &quot;'&quot; , &quot;logoutForm&quot; , &quot;'&quot; , &quot;).submit();
        }

        function removeUserSchoolPermissionsFromSessionStorage(){
            if (window?.eduWorx.currentUser.Id &amp;&amp; window?.eduWorx.currentUser.role){
                var key = window.eduWorx.currentUser.Id + &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; + window.eduWorx.currentUser.role + &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;SchoolPermissions&quot; , &quot;'&quot; , &quot;;
                sessionStorage.removeItem(key);
            }
        }

                eduWorx.currentUser.isLocalAccount = true;

         if (typeof angular != &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
             angular.element(document).ready(function () {
                 var showMoreRolesDiv = document.getElementById(&quot;showMoreRolesDiv&quot;);
                 if (showMoreRolesDiv) {
                     angular.module(&quot; , &quot;'&quot; , &quot;app.schoolSelector&quot; , &quot;'&quot; , &quot;);
                     try {
                         angular.bootstrap(showMoreRolesDiv, [&quot;app.schoolSelector&quot;]);
                     } catch (e) {

                     }

                 }
             });
         }


                
            &quot;))]</value>
      <webElementGuid>008c378c-0bf5-4f58-8464-ad0c0c0add1b</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
