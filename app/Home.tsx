/**
 * @format
 */

import { Appbar } from "react-native-paper";
import React from "react";

export default function Home() {
    return (
        <Appbar.Header>
            <Appbar.BackAction onPress={() => console.log("Pressed back")} />
            <Appbar.Content title="Babel Books" subtitle="Your Library" />
            <Appbar.Action
                icon="magnify"
                onPress={() => console.log("Pressed magnify")}
            />
            <Appbar.Action
                icon="dots-vertical"
                onPress={() => console.log("Pressed dots vertical")}
            />
        </Appbar.Header>
    );
}
