tts服务器已经完成部署，
api调用端口为：
http://192.168.11.153:8080/
api调用文档，官方文档
1. Confirm that you have cURL installed on your system.

copy
$ curl --version
2. Find the API endpoint below corresponding to your desired function in the app. Copy the code snippet, replacing the placeholder values with your own input data. Or use the 
API Recorder

 to automatically generate your API requests.
 
Making a prediction and getting a result requires 2 requests: a POST and a GET request. The POST request returns an EVENT_ID, which is used in the second GET request to fetch the results. In these snippets, we've used awk and read to parse the results, combining these two requests into one command for ease of use. See curl docs.

api_name: /on_upload_sample_audio
copy
curl -X POST http://192.168.11.153:8080/gradio_api/call/on_upload_sample_audio -s -H "Content-Type: application/json" -d '{
  "data": [
							{"path":"https://github.com/gradio-app/gradio/raw/main/test/test_files/audio_sample.wav","meta":{"_type":"gradio.FileData"}}
]}' \
  | awk -F'"' '{ print $4}'  \
  | read EVENT_ID; curl -N http://192.168.11.153:8080/gradio_api/call/on_upload_sample_audio/$EVENT_ID
Accepts 1 parameter:
[0] any Required

The input value that is provided in the "parameter_11" Audio component. The FileData class is a subclass of the GradioModel class that represents a file object within a Gradio interface. It is used to store file data and metadata when a file is uploaded. Attributes: path: The server file path where the file is stored. url: The normalized server URL pointing to the file. size: The size of the file in bytes. orig_name: The original filename before upload. mime_type: The MIME type of the file. is_stream: Indicates whether the file is a stream. meta: Additional metadata used internally (should not be changed).

Returns 1 element
string

The output value that appears in the "value_13" Textbox component.

api_name: /lambda
copy
curl -X POST http://192.168.11.153:8080/gradio_api/call/lambda -s -H "Content-Type: application/json" -d '{
  "data": [
]}' \
  | awk -F'"' '{ print $4}'  \
  | read EVENT_ID; curl -N http://192.168.11.153:8080/gradio_api/call/lambda/$EVENT_ID
Accepts 0 parameters:
Returns 1 element
api_name: /on_voice_change
copy
curl -X POST http://192.168.11.153:8080/gradio_api/call/on_voice_change -s -H "Content-Type: application/json" -d '{
  "data": [
							"Default"
]}' \
  | awk -F'"' '{ print $4}'  \
  | read EVENT_ID; curl -N http://192.168.11.153:8080/gradio_api/call/on_voice_change/$EVENT_ID
Accepts 1 parameter:
[0] string Required

The input value that is provided in the "Timbre" Dropdown component.

Returns 1 element
number

The output value that appears in the "Audio Seed" Number component.

api_name: /generate_seed
copy
curl -X POST http://192.168.11.153:8080/gradio_api/call/generate_seed -s -H "Content-Type: application/json" -d '{
  "data": [
]}' \
  | awk -F'"' '{ print $4}'  \
  | read EVENT_ID; curl -N http://192.168.11.153:8080/gradio_api/call/generate_seed/$EVENT_ID
Accepts 0 parameters:
Returns 1 element
number

The output value that appears in the "Audio Seed" Number component.

api_name: /generate_seed_1
copy
curl -X POST http://192.168.11.153:8080/gradio_api/call/generate_seed_1 -s -H "Content-Type: application/json" -d '{
  "data": [
]}' \
  | awk -F'"' '{ print $4}'  \
  | read EVENT_ID; curl -N http://192.168.11.153:8080/gradio_api/call/generate_seed_1/$EVENT_ID
Accepts 0 parameters:
Returns 1 element
number

The output value that appears in the "Text Seed" Number component.

api_name: /on_audio_seed_change
copy
curl -X POST http://192.168.11.153:8080/gradio_api/call/on_audio_seed_change -s -H "Content-Type: application/json" -d '{
  "data": [
							3
]}' \
  | awk -F'"' '{ print $4}'  \
  | read EVENT_ID; curl -N http://192.168.11.153:8080/gradio_api/call/on_audio_seed_change/$EVENT_ID
Accepts 1 parameter:
[0] number Required

The input value that is provided in the "Audio Seed" Number component.

Returns 1 element
string

The output value that appears in the "Speaker Embedding" Textbox component.

api_name: /reload_chat
copy
curl -X POST http://192.168.11.153:8080/gradio_api/call/reload_chat -s -H "Content-Type: application/json" -d '{
  "data": [
							"Hello!!"
]}' \
  | awk -F'"' '{ print $4}'  \
  | read EVENT_ID; curl -N http://192.168.11.153:8080/gradio_api/call/reload_chat/$EVENT_ID
Accepts 1 parameter:
[0] string Required

The input value that is provided in the "DVAE Coefficient" Textbox component.

Returns 1 element
string

The output value that appears in the "DVAE Coefficient" Textbox component.

api_name: /interrupt_generate
copy
curl -X POST http://192.168.11.153:8080/gradio_api/call/interrupt_generate -s -H "Content-Type: application/json" -d '{
  "data": [
]}' \
  | awk -F'"' '{ print $4}'  \
  | read EVENT_ID; curl -N http://192.168.11.153:8080/gradio_api/call/interrupt_generate/$EVENT_ID
Accepts 0 parameters:
Returns 1 element

JavaScript:
1. Install the javascript client (docs) if you don't already have it installed.

copy
$ npm i -D @gradio/client
2. Find the API endpoint below corresponding to your desired function in the app. Copy the code snippet, replacing the placeholder values with your own input data. Or use the 
API Recorder

 to automatically generate your API requests.

api_name: /on_upload_sample_audio
copy
import { Client } from "@gradio/client";

const response_0 = await fetch("https://github.com/gradio-app/gradio/raw/main/test/test_files/audio_sample.wav");
const exampleAudio = await response_0.blob();
						
const client = await Client.connect("http://192.168.11.153:8080/");
const result = await client.predict("/on_upload_sample_audio", { 
				sample_audio_input: exampleAudio, 
});

console.log(result.data);
Accepts 1 parameter:
sample_audio_input any Required

The input value that is provided in the "parameter_11" Audio component. The FileData class is a subclass of the GradioModel class that represents a file object within a Gradio interface. It is used to store file data and metadata when a file is uploaded. Attributes: path: The server file path where the file is stored. url: The normalized server URL pointing to the file. size: The size of the file in bytes. orig_name: The original filename before upload. mime_type: The MIME type of the file. is_stream: Indicates whether the file is a stream. meta: Additional metadata used internally (should not be changed).

Returns 1 element
string

The output value that appears in the "value_13" Textbox component.

api_name: /lambda
copy
import { Client } from "@gradio/client";

const client = await Client.connect("http://192.168.11.153:8080/");
const result = await client.predict("/lambda", { 
});

console.log(result.data);
Accepts 0 parameters:
Returns 1 element
api_name: /on_voice_change
copy
import { Client } from "@gradio/client";

const client = await Client.connect("http://192.168.11.153:8080/");
const result = await client.predict("/on_voice_change", { 		
		vocie_selection: "Default", 
});

console.log(result.data);
Accepts 1 parameter:
vocie_selection string Default: "Default"

The input value that is provided in the "Timbre" Dropdown component.

Returns 1 element
number

The output value that appears in the "Audio Seed" Number component.

api_name: /generate_seed
copy
import { Client } from "@gradio/client";

const client = await Client.connect("http://192.168.11.153:8080/");
const result = await client.predict("/generate_seed", { 
});

console.log(result.data);
Accepts 0 parameters:
Returns 1 element
number

The output value that appears in the "Audio Seed" Number component.

api_name: /generate_seed_1
copy
import { Client } from "@gradio/client";

const client = await Client.connect("http://192.168.11.153:8080/");
const result = await client.predict("/generate_seed_1", { 
});

console.log(result.data);
Accepts 0 parameters:
Returns 1 element
number

The output value that appears in the "Text Seed" Number component.

api_name: /on_audio_seed_change
copy
import { Client } from "@gradio/client";

const client = await Client.connect("http://192.168.11.153:8080/");
const result = await client.predict("/on_audio_seed_change", { 		
		audio_seed_input: 3, 
});

console.log(result.data);
Accepts 1 parameter:
audio_seed_input number Default: 2

The input value that is provided in the "Audio Seed" Number component.

Returns 1 element
string

The output value that appears in the "Speaker Embedding" Textbox component.

api_name: /reload_chat
copy
import { Client } from "@gradio/client";

const client = await Client.connect("http://192.168.11.153:8080/");
const result = await client.predict("/reload_chat", { 		
		coef: "Hello!!", 
});

console.log(result.data);
Accepts 1 parameter:
coef string Default: "爢涣蒏谾巉慧囿訁乏縌桌貨娉懣庤眾縺寃观論嶎覗滾亖噏虙笘趞蜖燼兕弽櫻苓访聕巁眍狹谋寏耍幈豼礡臨免甿貟聓貝琖巗縅拼没敏誀潠豔荝臣渥茿睅渃諡纃嵺椀拿穖恏簅糴譂彇臵缀眿刣盃虏灥巑媍嫹嘙媏虞毤趆磬燭狤栿刬綣舀仛岢粴囼咶菏硋惐貢獝懻賨丿桏勓赃媺嶪兇竻蚥觏揀撐賄叵凡茠昿熧舳话奖左癵拽吽艏瀅婬跐劲臡働笽蟡潣跙衫州屾嫷蓖勏赥蛘贋橏凴矨舿紅敳虙奃巔晷综誸哏详渐赨枑燡糥爿抰夣蛓峮嶅睒櫼衴挏耯襜谏準臲殼脿舠哃赳坚嶮仰替泍肏籷菸贱殑燰您儾殺滃谐睰嶸喤拵绝斏滨筰趕堋凼嗄洿縞正蛨彪巀㴁"

The input value that is provided in the "DVAE Coefficient" Textbox component.

Returns 1 element
string

The output value that appears in the "DVAE Coefficient" Textbox component.

api_name: /interrupt_generate
copy
import { Client } from "@gradio/client";

const client = await Client.connect("http://192.168.11.153:8080/");
const result = await client.predict("/interrupt_generate", { 
});

console.log(result.data);
Accepts 0 parameters:
Returns 1 element

Python:
1. Install the python client (docs) if you don't already have it installed.

$ pip install gradio_client

2. Find the API endpoint below corresponding to your desired function in the app. Copy the code snippet, replacing the placeholder values with your own input data. Or use the

to automatically generate your API requests.
api_name: /on_upload_sample_audio

from gradio_client import Client, handle_file

client = Client("http://192.168.11.153:8080/")
result = client.predict(
		sample_audio_input=handle_file('https://github.com/gradio-app/gradio/raw/main/test/test_files/audio_sample.wav'),
		api_name="/on_upload_sample_audio"
)
print(result)

Accepts 1 parameter:

sample_audio_input filepath Required

The input value that is provided in the "parameter_11" Audio component. The FileData class is a subclass of the GradioModel class that represents a file object within a Gradio interface. It is used to store file data and metadata when a file is uploaded. Attributes: path: The server file path where the file is stored. url: The normalized server URL pointing to the file. size: The size of the file in bytes. orig_name: The original filename before upload. mime_type: The MIME type of the file. is_stream: Indicates whether the file is a stream. meta: Additional metadata used internally (should not be changed).
Returns 1 element

str

The output value that appears in the "value_13" Textbox component.
api_name: /lambda

from gradio_client import Client

client = Client("http://192.168.11.153:8080/")
result = client.predict(
		api_name="/lambda"
)
print(result)

Accepts 0 parameters:
Returns 1 element
api_name: /on_voice_change

from gradio_client import Client

client = Client("http://192.168.11.153:8080/")
result = client.predict(
		vocie_selection="Default",
		api_name="/on_voice_change"
)
print(result)

Accepts 1 parameter:

vocie_selection Literal['Default', 'Timbre1', 'Timbre2', 'Timbre3', 'Timbre4', 'Timbre5', 'Timbre6', 'Timbre7', 'Timbre8', 'Timbre9'] Default: "Default"

The input value that is provided in the "Timbre" Dropdown component.
Returns 1 element

float

The output value that appears in the "Audio Seed" Number component.
api_name: /generate_seed

from gradio_client import Client

client = Client("http://192.168.11.153:8080/")
result = client.predict(
		api_name="/generate_seed"
)
print(result)

Accepts 0 parameters:
Returns 1 element

float

The output value that appears in the "Audio Seed" Number component.
api_name: /generate_seed_1

from gradio_client import Client

client = Client("http://192.168.11.153:8080/")
result = client.predict(
		api_name="/generate_seed_1"
)
print(result)

Accepts 0 parameters:
Returns 1 element

float

The output value that appears in the "Text Seed" Number component.
api_name: /on_audio_seed_change

from gradio_client import Client

client = Client("http://192.168.11.153:8080/")
result = client.predict(
		audio_seed_input=2,
		api_name="/on_audio_seed_change"
)
print(result)

Accepts 1 parameter:

audio_seed_input float Default: 2

The input value that is provided in the "Audio Seed" Number component.
Returns 1 element

str

The output value that appears in the "Speaker Embedding" Textbox component.
api_name: /reload_chat

from gradio_client import Client

client = Client("http://192.168.11.153:8080/")
result = client.predict(
		coef="爢涣蒏谾巉慧囿訁乏縌桌貨娉懣庤眾縺寃观論嶎覗滾亖噏虙笘趞蜖燼兕弽櫻苓访聕巁眍狹谋寏耍幈豼礡臨免甿貟聓貝琖巗縅拼没敏誀潠豔荝臣渥茿睅渃諡纃嵺椀拿穖恏簅糴譂彇臵缀眿刣盃虏灥巑媍嫹嘙媏虞毤趆磬燭狤栿刬綣舀仛岢粴囼咶菏硋惐貢獝懻賨丿桏勓赃媺嶪兇竻蚥觏揀撐賄叵凡茠昿熧舳话奖左癵拽吽艏瀅婬跐劲臡働笽蟡潣跙衫州屾嫷蓖勏赥蛘贋橏凴矨舿紅敳虙奃巔晷综誸哏详渐赨枑燡糥爿抰夣蛓峮嶅睒櫼衴挏耯襜谏準臲殼脿舠哃赳坚嶮仰替泍肏籷菸贱殑燰您儾殺滃谐睰嶸喤拵绝斏滨筰趕堋凼嗄洿縞正蛨彪巀㴁",
		api_name="/reload_chat"
)
print(result)

Accepts 1 parameter:

coef str Default: "爢涣蒏谾巉慧囿訁乏縌桌貨娉懣庤眾縺寃观論嶎覗滾亖噏虙笘趞蜖燼兕弽櫻苓访聕巁眍狹谋寏耍幈豼礡臨免甿貟聓貝琖巗縅拼没敏誀潠豔荝臣渥茿睅渃諡纃嵺椀拿穖恏簅糴譂彇臵缀眿刣盃虏灥巑媍嫹嘙媏虞毤趆磬燭狤栿刬綣舀仛岢粴囼咶菏硋惐貢獝懻賨丿桏勓赃媺嶪兇竻蚥觏揀撐賄叵凡茠昿熧舳话奖左癵拽吽艏瀅婬跐劲臡働笽蟡潣跙衫州屾嫷蓖勏赥蛘贋橏凴矨舿紅敳虙奃巔晷综誸哏详渐赨枑燡糥爿抰夣蛓峮嶅睒櫼衴挏耯襜谏準臲殼脿舠哃赳坚嶮仰替泍肏籷菸贱殑燰您儾殺滃谐睰嶸喤拵绝斏滨筰趕堋凼嗄洿縞正蛨彪巀㴁"

The input value that is provided in the "DVAE Coefficient" Textbox component.
Returns 1 element

str

The output value that appears in the "DVAE Coefficient" Textbox component.
api_name: /interrupt_generate

from gradio_client import Client

client = Client("http://192.168.11.153:8080/")
result = client.predict(
		api_name="/interrupt_generate"
)
print(result)

Accepts 0 parameters:
Returns 1 element