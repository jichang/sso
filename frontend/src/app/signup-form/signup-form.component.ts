import { Component, OnInit, Output, EventEmitter } from '@angular/core';
import { FormBuilder, FormGroup, FormControl, Validators } from '@angular/forms';
import { HttpClient } from '@angular/common/http';
import { User } from '../model';

@Component({
  selector: 'signup-form',
  templateUrl: './signup-form.component.html',
  styleUrls: ['./signup-form.component.css']
})
export class SignupFormComponent implements OnInit {
  user: FormGroup;
  @Output() result = new EventEmitter();

  constructor(private fb: FormBuilder, private http: HttpClient) {
    this.user = fb.group({
      username: ['', [Validators.compose([Validators.required, Validators.minLength(1)])]],
      password: ['', [Validators.compose([Validators.required, Validators.minLength(1)])]]
    });
  }

  ngOnInit() {
  }

  signup({ value, valid }: { value: User, valid: boolean }) {
    let params = {
      username: value.username,
      password: value.password,
    };

    this.http.post('/api/v1/signup', params)
      .subscribe((response: Response) => {
        this.result.emit(response)
      });
  }
}
