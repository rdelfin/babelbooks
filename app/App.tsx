/**
 * @format
 */

import React from "react";
import { Provider as PaperProvider } from "react-native-paper";
import Home from "./pages/Home";

export default function App() {
    return (
        <PaperProvider>
            <Home />
        </PaperProvider>
    );
}
