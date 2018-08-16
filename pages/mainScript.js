
/*****************************************
  Project   :   CS410 Final Project
  Written by:   Hassan Kaboyo
******************************************/


var translationMap = '';

var theHost = "http://localhost:8080/";
//var theHost = 'http://localhost:8080/swahili';
//var theHost2 = 'http://localhost:8080/scores';


//Loads the game server with default value
selectFunction("swahili");


function selectFunction(chosenOption){
  var tempObj = '';
  if (chosenOption === "swahili"){
    //location.reload();
    tempObj = makeServerRequest(chosenOption);
    loadWords(tempObj);
  }
  if (chosenOption === "spanish"){
    //location.reload();
    tempObj = makeServerRequest(chosenOption);
    loadWords(tempObj);
  }
}


//Ajax request to the server takes in the host path
function makeServerRequest(currentHost)
{
  var request = new XMLHttpRequest();

  request.onreadystatechange = function() {
    if (request.readyState === 4) {
        if (request.status === 200) {
          //console.log(request.responseText);
          translationMap = JSON.parse(request.responseText);
        }
    }
  };

  request.open('GET', theHost + currentHost, false );
  //request.open('GET', theHost + "swahili", false );
  request.send('Hello World');

  return translationMap;
}


//Loads the divs with the passed in json object(hash map)
function loadWords(quizMap) {

  var words = [];
  var meaning = [];

  for (var x in quizMap){
    words.push(x);
    meaning.push(quizMap[x]);
  }

  var itemID = ["item1", "item2", "item3", "item4", "item5"];
  var matchID = ["match1", "match2", "match3", "match4", "match5"];

  var count = 0;
  for (var i =0; i < 5; i++){
     //Populate words
     document.getElementById(itemID[i]).innerHTML = words[i];
     ++count;
     document.getElementById(matchID[i]).innerHTML = count + ".  &nbsp;" + meaning[i];
    }
}


function calculateScore() {

  var words = [];
  for (var y in translationMap){
    words.push(y);
  }

  var userAnsMap = { };   //Store user's response
  var inputBox = ["box1", "box2", "box3", "box4", "box5"];

  for (var i = 0; i < 5; i++){
    userAnsMap[(words[i])] = document.getElementById(inputBox[i]).value;
  }

  //Check if there's any empty box
  var j;
  for (j = 0; j < 5; j++) {
    var checkVal = document.getElementById(inputBox[j]).value;
    if ( checkVal === '' ){ break; }
  }

  if (j === 5)
    compareResult(userAnsMap);
  else
    alert("Please fill all boxes");
}


function compareResult(userAnsMap){

  var myRequest = new XMLHttpRequest();
  var userMap = '';

  myRequest.onreadystatechange = function() {
      if (myRequest.readyState === 4) {
          if (myRequest.status === 200) {

            console.log(myRequest.responseText);
            var retData = JSON.parse(myRequest.responseText);
            var theScore = retData[1];  //console.log(retData[1]);
            var feedBack = retData[0];  //console.log(retData[0]);
            document.getElementById("score").innerHTML = theScore;
            document.getElementById("textFeed").innerHTML = feedBack;

          }
      }
  };

  myRequest.open('POST', theHost + "scores", false);
  //myRequest.setRequestHeader('Content-Type', 'application/json'); 
  myRequest.setRequestHeader('Content-Type', 'text/plain');
  myRequest.send(JSON.stringify(userAnsMap));
}
