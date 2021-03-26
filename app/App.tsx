/**
 * @format
 */

import { StatusBar } from "expo-status-bar";
import React from "react";
import { StyleSheet, Text, View } from "react-native";
import { Provider as PaperProvider } from "react-native-paper";
import { Appbar } from "react-native-paper";
import Home from "./pages/Home";

export default function App() {
    return (
        <PaperProvider>
            <Home />
        </PaperProvider>
    );
}
