import { Injectable } from "@angular/core";
import { HttpClient, HttpHeaders } from "@angular/common/http";
import { BehaviorSubject } from "rxjs";

export enum PreferenceKey {
  SIGNIN_TOTP = 0,
  SIGNIN_WEBAUTHN = 1
}

export interface Preference {
  key: PreferenceKey;
  enabled: boolean;
  details?: any;
}

export interface PreferenceStore {
  preferences: Preference[];
}

export const PREFERENCES = [
  {
    key: PreferenceKey.SIGNIN_TOTP,
    title: "Enable TOTP"
  },
  {
    key: PreferenceKey.SIGNIN_WEBAUTHN,
    title: "Enable WebAuthn"
  }
];

@Injectable({
  providedIn: "root"
})
export class PreferenceService {
  store: PreferenceStore = {
    preferences: []
  };
  private subject: BehaviorSubject<Preference[]> = new BehaviorSubject([]);

  constructor(private http: HttpClient) {}

  get preferences() {
    return this.subject;
  }

  sync(userId: number) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/users/${userId}/preferences`;
    return this.http
      .get(apiUri, options)
      .subscribe((preferences: Preference[]) => {
        this.store.preferences = preferences;
        this.subject.next(preferences);
      });
  }

  update(userId: number, preference: Preference) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });

    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/users/${userId}/preferences`;
    return this.http
      .post(apiUri, preference, options)
      .subscribe((preference: Preference) => {
        let index = this.store.preferences.findIndex(
          _preference => _preference.key === preference.key
        );
        this.store.preferences[index] = preference;
        this.subject.next(this.store.preferences);
      });
  }
}
