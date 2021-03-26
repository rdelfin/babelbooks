/**
 * @format
 */

import { Appbar } from "react-native-paper";
import { StyleSheet, Text, View } from "react-native";
import React from "react";

export default function Home() {
    return (
        <Appbar style={styles.bottom}>
            <Appbar.Action
                icon="archive"
                onPress={() => console.log("Pressed archive")}
            />
            <Appbar.Action
                icon="mail"
                onPress={() => console.log("Pressed mail")}
            />
            <Appbar.Action
                icon="label"
                onPress={() => console.log("Pressed label")}
            />
            <Appbar.Action
                icon="delete"
                onPress={() => console.log("Pressed delete")}
            />
        </Appbar>
    );
}

const styles = StyleSheet.create({
    bottom: {
        position: "absolute",
        left: 0,
        right: 0,
        top: 0,
    },
});
