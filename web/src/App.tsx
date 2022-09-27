import React from "react"

import "./tailwind.css"

const App = () => {
    return (
        <div className="App bg-[#eee] h-[100vh] flex items-center justify-center flex-col">
            <h1 className="font-bold text-4xl text-[#333]">Hi 👋, this is a test file to verify that Tailwind is working properly.</h1>
            <p className="text-sm font-italic m-[3px]">
                If this is italicized, then you're good to go!
            </p>
        </div>
    )
}

export default App