import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, FormControl, Validators } from '@angular/forms';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { ActivatedRoute } from '@angular/router';
import { Contact, ContactType, ContactModelService } from '../contact-model.service';
import { session } from '../model';

interface Verification {
  contact_id: Contact;
}

@Component({
  selector: 'verify-form',
  templateUrl: './verify-form.component.html',
  styleUrls: ['./verify-form.component.css']
})
export class VerifyFormComponent implements OnInit {
  verification: FormGroup;
  contacts: Contact[] = [];

  constructor(private fb: FormBuilder, private http: HttpClient, private route: ActivatedRoute) {
    this.verification = fb.group({
      contact_id: [this.route.snapshot.queryParams['contact_id'], [Validators.required]]
    });
  }

  ngOnInit() {
    this.queryContacts();
  }

  queryContacts() {
    let headers = new HttpHeaders({
      'Content-Type': 'application/json',
      'Authorization': 'Bearer ' + window.localStorage.getItem('jwt')
    });
    let options = {
      headers: headers
    };

    let user = session.currUser();
    let apiUri = '/api/v1/users/' + user.id + '/contacts';
    this.http.get(apiUri, options)
      .subscribe((response: any) => {
        this.contacts = response;
      });
  };

  verify({ value, valid }: { value: Verification, valid: boolean }) {
    let headers = new HttpHeaders({
      'Content-Type': 'application/json',
      'Authorization': 'Bearer ' + window.localStorage.getItem('jwt')
    });
    let options = {
      headers: headers
    };

    let user = session.currUser();
    let apiUri = '/api/v1/users/' + user.id + '/contacts/' + value.contact_id + '/verifications';
    this.http.post(apiUri, {}, options)
      .subscribe((response: Response) => {
        console.log(response);
      });
  }
}
