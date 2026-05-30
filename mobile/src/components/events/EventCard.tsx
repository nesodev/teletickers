import React, { useRef, useCallback } from 'react';
import { View, Text, StyleSheet, Pressable } from 'react-native';
import { Image } from 'expo-image';
import BottomSheet, { BottomSheetBackdrop } from '@gorhom/bottom-sheet';
import { Feather } from '@expo/vector-icons';

export interface EventCardProps {
  id: string;
  title: string;
  date: string;
  imageUrl: string;
  onPress: () => void;
}

export const EventCard = ({ title, date, imageUrl, onPress }: EventCardProps) => {
  const bottomSheetRef = useRef<BottomSheet>(null);

  const handleOpenMenu = useCallback(() => {
    bottomSheetRef.current?.expand();
  }, []);

  const renderBackdrop = useCallback(
    (props: any) => (
      <BottomSheetBackdrop {...props} disappearsOnIndex={-1} appearsOnIndex={0} />
    ),
    []
  );

  return (
    <>
      <Pressable style={styles.card} onPress={onPress}>
        <Image source={{ uri: imageUrl }} style={styles.image} contentFit="cover" />
        <View style={styles.content}>
          <View style={styles.textContainer}>
            <Text style={styles.title} numberOfLines={1}>{title}</Text>
            <Text style={styles.date}>{date}</Text>
          </View>
          <Pressable style={styles.menuButton} onPress={handleOpenMenu} hitSlop={10}>
            <Feather name="more-vertical" size={20} color="#64748b" />
          </Pressable>
        </View>
      </Pressable>

      <BottomSheet
        ref={bottomSheetRef}
        index={-1}
        snapPoints={['25%']}
        backdropComponent={renderBackdrop}
        enablePanDownToClose
      >
        <View style={styles.sheetContent}>
          <Text style={styles.sheetTitle}>Acciones</Text>
          <Pressable style={styles.sheetItem}>
            <Text style={styles.sheetItemText}>Publicar Evento</Text>
          </Pressable>
          <Pressable style={[styles.sheetItem, styles.cancelItem]}>
            <Text style={[styles.sheetItemText, styles.cancelText]}>Cancelar Evento</Text>
          </Pressable>
        </View>
      </BottomSheet>
    </>
  );
};

const styles = StyleSheet.create({
  card: {
    backgroundColor: '#ffffff',
    borderRadius: 16,
    marginHorizontal: 16,
    marginBottom: 16,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.05,
    shadowRadius: 8,
    elevation: 2,
    overflow: 'hidden',
  },
  image: {
    width: '100%',
    height: 160,
  },
  content: {
    padding: 16,
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  textContainer: {
    flex: 1,
    marginRight: 16,
  },
  title: {
    fontSize: 18,
    fontWeight: '600',
    color: '#0f172a',
    marginBottom: 4,
  },
  date: {
    fontSize: 14,
    color: '#64748b',
  },
  menuButton: {
    padding: 4,
  },
  sheetContent: {
    flex: 1,
    padding: 24,
  },
  sheetTitle: {
    fontSize: 18,
    fontWeight: '700',
    marginBottom: 16,
    color: '#0f172a',
  },
  sheetItem: {
    paddingVertical: 16,
    borderBottomWidth: 1,
    borderBottomColor: '#f1f5f9',
  },
  sheetItemText: {
    fontSize: 16,
    color: '#334155',
  },
  cancelItem: {
    borderBottomWidth: 0,
  },
  cancelText: {
    color: '#ef4444',
  },
});
