import { gql } from '@apollo/client';

export const GET_EVENTS = gql`
  query GetEvents {
    events {
      id
      title
      date
      imageUrl
    }
  }
`;

export const GET_MY_EVENTS = gql`
  query GetMyEvents {
    myEvents {
      id
      title
      date
      imageUrl
    }
  }
`;

export const GET_EVENT_DETAIL = gql`
  query GetEventDetail($id: ID!) {
    event(id: $id) {
      id
      title
      description
      date
      imageUrl
    }
  }
`;
