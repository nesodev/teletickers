import { ApolloProvider } from '@apollo/client/react';
import { client } from '../../lib/apollo-client';
import type { ReactNode } from 'react';

interface Props {
  children: ReactNode;
}

export default function ApolloWrapper({ children }: Props) {
  return <ApolloProvider client={client}>{children}</ApolloProvider>;
}