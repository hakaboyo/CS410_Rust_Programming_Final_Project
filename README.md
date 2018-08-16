

# Written by: Hassan Kaboyo
# Project:    Language Web Application
# Date:		  8/13/2018



## 	Description
	
	This small web app is intended for those who are interested in learning 
	multiple languages other than the language they already use(English). This 
	application works by displaying two sets of words to the user where one set 
	contains English words while other set contains words from a language that 
	a user can select. Currently the language contains Swahili and Spanish. 
	After the two sets are displayed, the user continue by matching the words 
	from one set to the other, then submit the answer. After evalueation the 
	application displays the feedback to the user. The feedback shows words that
	were correctly matched.
	
##	How To Use The Application

	# 	Option 1:
	
		Under current settings, User may navigate to the html file located in 
		"webapp/pages/GameProject.html"  then open it with a web browser, google
		chrome works exelent. The app should load Swahili-English dictionary by 
		default, then after matching the words pair, the submit button will bring 
		results.
	
	# 	Option 2:
	
		Navigate to the "webapp" project folder then use command "cargo run" to start
		the server. then on the browser you may start with  http://localhost:8080,
		and this should give a commplete path to the application or you may go 
		straight to the app with http://localhost:8080/home/GameProject.html
	
	
##	Functionality

	At page load, the application will makes a GET request from the server which  
	then respond with a string keys to string values HashMap sent as Json object,  
	then load the page with default language or (English  words  to  be  matched  
	with Swahili words). Also a user can make language selection after page load.  
	The JSON object has been passed to the function that scrambles its key's values  
	so that user can try to reassemble them correctly then hit submit when done. 
	
	The submit button action calls a Java Script method that collects user's answer
	back to a Json dictionary object then makes a POST request to the server with 
	that object as post data for evaluation of user's answer. Eventually the server  
	responds with feedback. On the server side, after receiving a POST request,  
	the server calls the function that compares users answers against the HashMap
	stored as a shared resource amognst the server's middlewares.
	
	As long as the server is running, Each middleware keeps a reference to this 
	HashMap that contains un-shuffled recently requested JSON object. This should 
	always contain the correct key-value mapping.
	
	
##	Language And Features

	The front end is mainly made of HTML that displays all of the divs components  
	such as words display, input boxes for user input, buttons and  drop-down menu 
	for language selection. It contains CSS for applying some responsiveness as well  
	as appropriate styling in different places of the web. All of server requests are
	made with JavaScript. The Server is made Rust as core language in conjunction 
	with Iron library crates. The server is what serves the randomly built question’s 
	HashMap according to the user's language choice, currently there are just two,  
	English and Swahili. I could not use any database tools like Mongo db or anything
	else due to time constraints. 
	
	Therefore each of the requested HashMap objects is built from two sets of words
	stored in an &str type vectors. For each Language there are two vectors where one
	vector contain English translation words and the other contains words of other 
	language. For each language, both words vector and word translation vector can be
	of any length, however both must have same length, Otherwise the function that 
	builds a hashmap from them will cause the program to panic.
	
	
	
	