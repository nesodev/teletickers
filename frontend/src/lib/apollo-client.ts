import { ApolloClient, InMemoryCache, HttpLink, ApolloLink } from '@apollo/client';

const httpLink = new HttpLink({
  uri: import.meta.env.PUBLIC_GRAPHQL_URL || 'http://localhost:8080/graphql',
});

const authLink = new ApolloLink((operation, forward) => {
  if (typeof window !== 'undefined') {
    const token = localStorage.getItem('token');
    operation.setContext({
      headers: {
        authorization: token ? `Bearer ${token}` : '',
      },
    });
  }
  return forward(operation);
});

export const client = new ApolloClient({
  link: authLink.concat(httpLink),
  cache: new InMemoryCache(),
  defaultOptions: {
    watchQuery: {
      fetchPolicy: 'network-only',
    },
  },
});