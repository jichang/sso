import { Component, OnInit, Output, EventEmitter } from '@angular/core';
import { FormBuilder, FormGroup, FormControl, Validators } from '@angular/forms';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Contact, ContactType, ContactModelService } from '../contact-model.service';
import { session } from '../model';

@Component({
  selector: 'contact-form',
  templateUrl: './contact-form.component.html',
  styleUrls: ['./contact-form.component.css']
})
export class ContactFormComponent implements OnInit {
  contact: FormGroup;
  @Output() result = new EventEmitter();

  constructor(private fb: FormBuilder, private http: HttpClient) {
    this.contact = fb.group({
      identity: ['', [Validators.required]],
    });
  }

  ngOnInit() {
  }

  create({ value, valid }: { value: Contact, valid: boolean }) {
    let headers = new HttpHeaders({
      'Content-Type': 'application/json',
      'Authorization': 'Bearer ' + window.localStorage.getItem('jwt')
    });
    let options = {
      headers: headers
    };

    value['type_id'] = ContactType.Email;
    let user = session.currUser();
    let apiUri = '/api/v1/users/' + user.id + '/contacts';
    this.http.post(apiUri, value, options)
      .subscribe((response: Response) => {
        this.result.emit(response);
      });
  }
}
