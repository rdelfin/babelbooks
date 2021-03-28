/**
 * @format
 */

import { Appbar } from "react-native-paper";
import React from "react";
import BookCard from "../components/book_card";
import { StyleSheet, View, ScrollView } from "react-native";

const styles = StyleSheet.create({
    scrollView: {
        paddingLeft: 20,
        paddingRight: 20,
        paddingTop: 20,
        paddingBottom: 0,
        marginBottom: 100,
    },
});

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
            <ScrollView style={styles.scrollView}>
                <BookCard
                    title="Jurassic Park"
                    authors={["Michael Crichton"]}
                    thumbnail_url="http://books.google.com/books/content?id=DYfxuAEACAAJ&printsec=frontcover&img=1&zoom=5&source=gbs_api"
                />
                <BookCard
                    title="Jurassic Park"
                    authors={["Michael Crichton"]}
                    thumbnail_url="http://books.google.com/books/content?id=DYfxuAEACAAJ&printsec=frontcover&img=1&zoom=5&source=gbs_api"
                />
            </ScrollView>
        </View>
    );
}
