import { useState,useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
    const [formatedStr, setFormatedStr] = useState("");
    const [dataStr, setDataStr] = useState("");
    const [fileType, setFiletype] = useState("json");

    async function beautify() {
        if (dataStr !== "") {
            const cleanedStr = dataStr
                .replaceAll('\u201C', '"')  // " (left double quote)
                .replaceAll('\u201D', '"'); // " (right double quote)

            setDataStr(cleanedStr);


            switch (fileType) {
                case "json":{
                    //uses cleanedStr not dataStr due to weird behavior regarding async
                    let updatedStr = await invoke("beautify", { dataStr: cleanedStr, fileType: "json" });
                    setFormatedStr(updatedStr);
                    break;
                }
                case "xml":{
                    let updatedStr = await invoke("beautify", { dataStr: cleanedStr, fileType: "xml" });
                    setFormatedStr(updatedStr);
                    break;
                }
                case "toml":{
                    let updatedStr = await invoke("beautify", { dataStr: cleanedStr, fileType: "toml" });
                    setFormatedStr(updatedStr);
                    break;
                }
                case "yml":{
                    let updatedStr = await invoke("beautify", { dataStr: cleanedStr, fileType: "yml" });
                    setFormatedStr(updatedStr);
                    break;
                }
                case "markdown":{
                    let updatedStr = await invoke("beautify", { dataStr: cleanedStr, fileType: "markdown" });
                    setFormatedStr(updatedStr);
                    break;
                }
                default:{
                    break;
                }
            }

        }
    }

    //handles main change of text field
    function handleChange(e) {
        setDataStr(e.target.value);
    }

    function handleType(e){
        setFiletype(e.target.value);
    }

    //Todo: Move to css file after confirming
    const textStyle= {
        height: '75vh',
        resize: "none",
        backgroundColor: "#c8d7db",
        fontSize: '.78rem'//Todo Urgent: Get feedback on height, and font size
    }

    //On Enter beautify
    document.addEventListener('keydown', (e) => {
        if ((e.key === 'Enter' && e.metaKey) || (e.key === 'Enter' && e.ctrlKey)) {
            beautify();
        }
    });


    return (
        <main className="container">
            <form>
                <div className="row">
                    <textarea style={textStyle} onChange={handleChange} id={"left"} value={dataStr}
                              className="column"></textarea>
                    <textarea style={textStyle} id={"right"} value={formatedStr} name={"rightText"}
                              className="column"></textarea>
                </div>
            </form>

            <footer className={"bottom-bar"}>
                <input className={"bottom-item"} value={"Beautify"} type={"button"} onClick={beautify}/>
                <select onChange={handleType} className={"bottom-item"} name="type" id="filetype">
                    <option value="json">Json</option>
                    <option value="toml">TOML</option>
                    <option value="markdown">Markdown</option>
                    <option value="yml">Yaml</option>
                    <option value="xml">XML</option>
                </select>

                <input className={"bottom-item"} value={"Copy"} type={"button"} style={{marginLeft: "auto"}} onClick={beautify}/>
            </footer>
        </main>

    );
}

export default App;
