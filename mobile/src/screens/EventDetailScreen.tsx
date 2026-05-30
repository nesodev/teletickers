import React from 'react';
import { View, Text, StyleSheet, ScrollView, SafeAreaView, Pressable } from 'react-native';
import { Image } from 'expo-image';

export const EventDetailScreen = () => {
  return (
    <SafeAreaView style={styles.container}>
      <ScrollView contentContainerStyle={styles.scrollContent}>
        <Image source={{ uri: 'https://via.placeholder.com/800x400' }} style={styles.cover} contentFit="cover" />
        <View style={styles.details}>
          <Text style={styles.title}>Detalle del Evento</Text>
          <Text style={styles.date}>Próximamente</Text>
          <Text style={styles.description}>
            Descripción detallada del evento irá aquí.
          </Text>
        </View>
      </ScrollView>
      <View style={styles.footerAction}>
        <Pressable style={styles.primaryButton}>
          <Text style={styles.primaryButtonText}>Comprar Entradas</Text>
        </Pressable>
      </View>
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: '#ffffff' },
  scrollContent: { paddingBottom: 100 },
  cover: { width: '100%', height: 250 },
  details: { padding: 24 },
  title: { fontSize: 28, fontWeight: '800', color: '#0f172a', marginBottom: 8 },
  date: { fontSize: 16, color: '#3b82f6', fontWeight: '600', marginBottom: 16 },
  description: { fontSize: 16, color: '#475569', lineHeight: 24 },
  footerAction: {
    position: 'absolute',
    bottom: 0,
    left: 0,
    right: 0,
    padding: 24,
    backgroundColor: '#ffffff',
    borderTopWidth: 1,
    borderTopColor: '#f1f5f9',
  },
  primaryButton: {
    backgroundColor: '#0f172a',
    paddingVertical: 16,
    borderRadius: 12,
    alignItems: 'center',
  },
  primaryButtonText: { color: '#ffffff', fontSize: 16, fontWeight: '700' },
});
