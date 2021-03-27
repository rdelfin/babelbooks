/**
 * @format
 */

import React from "react";
import { Avatar, Button, Card, Title, Paragraph } from "react-native-paper";

export interface Props {
    title: string;
    authors: string[];
    thumbnail_url: string;
}

const BookCard: React.FC<Props> = (props) => {
    return (
        <Card elevation={10}>
            <Card.Cover source={{ uri: props.thumbnail_url }} />
            <Card.Title
                title={props.title}
                subtitle={props.authors.join(", ")}
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

export default BookCard;
