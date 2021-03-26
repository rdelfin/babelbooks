/**
 * @format
 */

import { Appbar } from "react-native-paper";
import React from "react";
import BookCard from "../components/book_card";
import { View } from "react-native";

export default function Home() {
    return (
        <View>
            <Appbar.Header>
                <Appbar.BackAction
                    onPress={() => console.log("Pressed back")}
                />
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
            <BookCard />
        </View>
    );
}
