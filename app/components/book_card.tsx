/**
 * @format
 */

import React from "react";
import { Avatar, Button, Card, Title, Paragraph } from "react-native-paper";

export default function BookCard() {
    return (
        <Card elevation={10}>
            <Card.Cover source={{ uri: "https://picsum.photos/700" }} />
            <Card.Title
                title="Card Title"
                subtitle="Card Subtitle"
            />
            <Card.Content>
                <Paragraph>Card content</Paragraph>
            </Card.Content>
            <Card.Actions>
                <Button>Cancel</Button>
                <Button>Ok</Button>
            </Card.Actions>
        </Card>
    );
}
