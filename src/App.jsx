import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [formatedStr, setFormatedStr] = useState("");
  const [dataStr, setDataStr] = useState("");

  async function beautify() {
    //   name = JSON.stringify(name,null));
      let updatedStr =await invoke("beautify", { dataStr})

      console.log("Updated STR: "+ updatedStr)
      console.log("--------")
      console.error(updatedStr)



      setFormatedStr(updatedStr);
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
                <input type={"button"} onClick={beautify} />
            </div>
    </main>

  );
}

// <h1>Beautify </h1>
// <form
//     className="row"
//     onSubmit={(e) => {
//         e.preventDefault();
//         beautify();
//     }}
// >
//     <input
//         id="greet-input"
//         onChange={(e) => setDataStr(e.currentTarget.value)}
//         placeholder="STRING GOES HERE"
//     />
//     <button type="submit">Format</button>
//     {/*<button onClick={()=>setDataStr("")} type="submit">Clear</button>*/}
// </form>
// <p>{formatedStr}</p>
export default App;
