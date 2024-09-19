# clip-clip
Clip polygons and multipolygons in geojson format

## Schemas
### input
```TS
type chopchopSubject = {
    _id?: string,
    area: Geometry
  };

type ChopchopBody = {
  areaToBeCovered: chopchopSubject,
  intersectingCandidates: chopchopSubject[]
};
```
### Output
if failed or no cover return string?
```TS
type ChopchopCover = {
  "covered%": number,
  coveredArea?: Geometry,
  leftover?: Geometry
  partialCoverages?: {
    "covered%": number,
    coveredArea?: Geometry,
    leftover?: Geometry
  }[]
}

```

partial coverages is the coverage for each product separately with the demand.

I'm not sure this is required, but I might calculate this nonetheless just to be sure.
