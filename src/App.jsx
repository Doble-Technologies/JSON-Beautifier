import { useState,useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [formatedStr, setFormatedStr] = useState("");
  const [dataStr, setDataStr] = useState("");
  
  async function beautify() {
    //   name = JSON.stringify(name,null));
      // ” is a smart quote " is the right one
      if(dataStr !==""){
          setDataStr(dataStr.replace('“', '"'))
          let updatedStr =await invoke("beautify", { dataStr: dataStr, fileType: "json"})
          setFormatedStr(updatedStr);
      }

  }

    function handleChange(e) {
        setDataStr(e.target.value);
    }

    return (
    <main className="container">
        <form >
            <div className="row">
                <textarea style={{minHeight: '75vh'}}  onChange={handleChange} id={"left"} value={dataStr} className="column"></textarea>
                <textarea  style={{minHeight: '75vh'}} id={"right"} value={formatedStr} name={"rightText"} className="column"></textarea>
            </div>
        </form>

        <div className={"bottom-bar"}>
                <input value={"Beautify"} type={"button"} onClick={beautify} />
                <select name="type" id="type">
                    <option value="json">Json</option>
                    <option value="toml">TOML</option>
                    <option value="markdown">Markdown</option>
                    <option value="yml">Yaml</option>
                    <option value="xml">XML</option>
                </select>
            </div>
    </main>

  );
}
export default App;
