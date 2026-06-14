import { gql } from '@apollo/client';

export const LOGIN_MUTATION = gql`
  mutation Login($email: String!, $password: String!) {
    login(email: $email, password: $password) {
      user {
        id
        nombre
        email
        dni
      }
      token
    }
  }
`;

export const REGISTER_MUTATION = gql`
  mutation Register(
    $nombre: String!
    $email: String!
    $password: String!
    $dni: String!
    $numeroCel: String
  ) {
    register(
      nombre: $nombre
      email: $email
      password: $password
      dni: $dni
      numeroCel: $numeroCel
    ) {
      user {
        id
        nombre
        email
        dni
      }
      token
    }
  }
`;

export const CREATE_EVENTO = gql`
  mutation CreateEvento(
    $titulo: String!
    $descripcion: String
    $fecha: String!
    $hora: String!
    $region: String!
    $provincia: String!
    $distrito: String!
    $categoria: String!
    $aforo: Int!
    $etiquetas: [String!]
    $restriccionEdad: String
    $miniatura: String
  ) {
    createEvento(
      titulo: $titulo
      descripcion: $descripcion
      fecha: $fecha
      hora: $hora
      region: $region
      provincia: $provincia
      distrito: $distrito
      categoria: $categoria
      aforo: $aforo
      etiquetas: $etiquetas
      restriccionEdad: $restriccionEdad
      miniatura: $miniatura
    ) {
      id
      titulo
      descripcion
      fecha
      hora
      estado
    }
  }
`;