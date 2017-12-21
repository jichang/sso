import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, FormControl, Validators } from '@angular/forms';
import { Response, Headers, RequestOptions } from '@angular/http';
import { Router, ActivatedRoute } from '@angular/router';
import { Application, ApplicationModelService } from '../application-model.service';
import { session } from '../model';

@Component({
  selector: 'application-form',
  templateUrl: './application-form.component.html',
  styleUrls: ['./application-form.component.css']
})
export class ApplicationFormComponent implements OnInit {
  application: FormGroup;

  constructor(private fb: FormBuilder, private applicationModelService: ApplicationModelService, private router: Router) {
    this.application = fb.group({
      name: ['', [Validators.required]],
      website_uri: ['', [Validators.required]],
      callback_uri: ['', [Validators.required]]
    });
  }

  ngOnInit() {
  }

  create({ value, valid }: { value: Application, valid: boolean }) {
    this.applicationModelService.create(value)
      .subscribe((application: Application) => {
        this.router.navigate(['dashboard/applications/' + application.id]);
      });
  }
}
