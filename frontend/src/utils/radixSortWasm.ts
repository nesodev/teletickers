export async function sortEventsByAforo(events: any[], ascending = true): Promise<any[]> {
  const eventsWithIndex = events.map((event, index) => ({
    event,
    aforo: event.aforo || 0,
    originalIndex: index
  }));

  eventsWithIndex.sort((a, b) => {
    if (ascending) {
      return a.aforo - b.aforo;
    } else {
      return b.aforo - a.aforo;
    }
  });

  return eventsWithIndex.map(item => item.event);
}

export async function sortEventsByPrice(events: any[], ascending = true): Promise<any[]> {
  const eventsWithIndex = events.map((event, index) => ({
    event,
    precio: event.precioMinimo || 0,
    originalIndex: index
  }));

  eventsWithIndex.sort((a, b) => {
    if (ascending) {
      return a.precio - b.precio;
    } else {
      return b.precio - a.precio;
    }
  });

  return eventsWithIndex.map(item => item.event);
}